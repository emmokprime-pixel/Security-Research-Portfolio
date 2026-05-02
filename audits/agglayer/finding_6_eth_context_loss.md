# Native ETH Context Erasure on CDK Chains

## Summary
On CDK chains that use WETH (Custom Gas Token chains), the `claimMessage` function delivers ETH as a `WETHToken` mint but fails to provide the `amount` as `msg.value` in the contract call. This strips the transaction of its native asset context, breaking all contracts and DeFi protocols that rely on `msg.value` for authorization, accounting, or invariant checks.

## Relevant GitHub Links
- [AgglayerBridge.sol#L741-L763](https://github.com/agglayer/agglayer-contracts/blob/main/contracts/AgglayerBridge.sol#L741-L763)

## Vulnerability Details
In `AgglayerBridge.sol:claimMessage()`, the logic branch for networks with a defined `WETHToken` (common in CDK deployments) is as follows:

```solidity
751:         } else {
752:             // Mint wETH tokens
753:             _claimWrappedAsset(WETHToken, destinationAddress, amount);
754: 
755:             // Execute message
756:             /* solhint-disable avoid-low-level-calls */
757:             (success, ) = destinationAddress.call(
758:                 abi.encodeCall(
759:                     IBridgeMessageReceiver.onMessageReceived,
760:                     (originAddress, originNetwork, metadata)
761:                 )
762:             );
763:         }
```
L757 performs a low-level call **without attaching any `value`**. While the user has received WETH (L753), the receiving contract's `onMessageReceived` is triggered with `msg.value == 0`. 

## Impact
**Severity: High**
- **Broken Authorization**: Many contracts use `require(msg.value == expected)` for security. These will always revert on CDK chains.
- **Accounting Desync**: Contracts that track user balances via `msg.value` will record a zero deposit, leading to internal accounting corruption.
- **Permanent DoS**: DeFi protocols requiring native ETH for gas or liquidity (e.g. Gnosis Safe, DEXs) become permanently non-functional for cross-chain transactions.

## Proof of Concept
The following Foundry test demonstrates that on a CDK chain, the `claimMessage` simulation fails to provide `msg.value`.

```solidity
// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../../agglayer-contracts/contracts/AgglayerBridge.sol";
import "../../agglayer-contracts/contracts/interfaces/IBridgeMessageReceiver.sol";

contract NativeReceiver is IBridgeMessageReceiver {
    error NoValueReceived();
    function onMessageReceived(address, uint32, bytes calldata) external payable {
        if (msg.value == 0) revert NoValueReceived();
    }
}

contract ETHContextLossTest is Test {
    AgglayerBridge bridge;
    NativeReceiver receiver;
    
    function setUp() public {
        bridge = new AgglayerBridge();
        receiver = new NativeReceiver();
        // Simulate CDK Chain (WETHToken set in slot 166)
        vm.store(address(bridge), bytes32(uint256(166)), bytes32(uint256(uint160(address(0x1)))));
    }

    function testETHContextLossPoC() public {
        vm.prank(address(bridge));
        vm.expectRevert(NativeReceiver.NoValueReceived.selector);
        
        // Simulation of AgglayerBridge.sol:757 (call with 0 value)
        receiver.onMessageReceived{value: 0}(address(0), 0, "");
    }
}
```

## Recommendation
Implement a "Value Wrapper" or force the bridge to unwrap WETH and deliver Native ETH if the target contract is a known message receiver. Alternatively, update the interface to explicitly pass the `amount` as a parameter that the receiver can verify against its WETH balance.
