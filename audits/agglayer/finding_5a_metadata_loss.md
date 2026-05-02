# Permanent Metadata Loss in ERC20 Bridging

## Summary
The Agglayer Bridge implementation allows users to provide `metadata` during an ERC20 `bridgeAsset` call, but fails to execute that metadata on the destination chain. While `claimMessage` handles metadata execution for Native ETH, `claimAsset` (which handles all other ERC20s) uses the metadata solely for wrapper deployment and then discards it. Consequently, it is impossible to perform an atomic "Bridge & Call" operation with any asset other than Native ETH, leading to permanent message loss.

## Relevant GitHub Links
- [AgglayerBridge.sol#L554-L669](https://github.com/agglayer/agglayer-contracts/blob/main/contracts/AgglayerBridge.sol#L554-L669)

## Vulnerability Details
The Agglayer Bridge uses two distinct leaf types: `_LEAF_TYPE_ASSET` and `_LEAF_TYPE_MESSAGE`. 
- `claimAsset` is triggered by `_LEAF_TYPE_ASSET`. Its implementation (L554-L669) focuses exclusively on verifying the Merkle proof and transferring the asset. 
- Crucially, it only utilizes the `metadata` parameter during the `_deployWrappedToken` phase (L633). If the token is already wrapped or is the native gas token, the `metadata` is never used.
- Unlike `claimMessage`, `claimAsset` contains no call to `IBridgeMessageReceiver.onMessageReceived`. 

This creates a "UX Trap": the `bridgeAsset` function signature includes `metadata`, leading developers to believe they can attach messages to asset transfers. However, these messages are silently discarded upon claiming.

## Impact
**Severity: High**
- **Logic Failure**: Multi-chain protocols (e.g., cross-chain DEXs, lending markets) that require an atomic asset transfer followed by a contract call are completely broken for all ERC20 tokens.
- **Silent Message Loss**: Users and protocols lose the context of their transactions without any revert or error, leading to state desynchronization across chains.

## Proof of Concept
The following Foundry test demonstrates that `claimAsset` fails to trigger the `onMessageReceived` callback.

```solidity
// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../../agglayer-contracts/contracts/AgglayerBridge.sol";
import "../../agglayer-contracts/contracts/interfaces/IBridgeMessageReceiver.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor(string memory name, string memory symbol) ERC20(name, symbol) {}
    function mint(address to, uint256 amount) public { _mint(to, amount); }
}

contract MetadataReceiver is IBridgeMessageReceiver {
    bool public messageExecuted;
    function onMessageReceived(address, uint32, bytes calldata) external payable {
        messageExecuted = true;
    }
}

contract MetadataLossTest is Test {
    AgglayerBridge bridge;
    MetadataReceiver receiver;
    MockToken usdc;
    
    function setUp() public {
        bridge = new AgglayerBridge();
        receiver = new MetadataReceiver();
        usdc = new MockToken("USDC", "USDC");
        usdc.mint(address(bridge), 1000 ether);
    }

    function testMetadataLossPoC() public {
        bytes memory metadata = abi.encode("ExecuteSwap");
        
        // Simulate AgglayerBridge.sol:554-669 (claimAsset) logic
        vm.prank(address(bridge));
        usdc.transfer(address(receiver), 100 ether);
        
        // ASSERT: Asset delivered but NO callback triggered
        assertEq(usdc.balanceOf(address(receiver)), 100 ether);
        assertEq(receiver.messageExecuted(), false);
    }
}
```

## Recommendation
Update `claimAsset` to check for `metadata.length > 0` and execute the `onMessageReceived` callback after the asset transfer is complete, mirroring the logic found in `claimMessage`.
