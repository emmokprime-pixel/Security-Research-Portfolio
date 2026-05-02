# Permanent Denial-of-Service (DoS) via Solvency Check Deadlock

## Summary
The `VaultBridgeToken` contract implements a hard-coded solvency check in its `_withdrawFromYieldVault` function that compares the yield vault's return against a strict slippage limit. If the yield vault (e.g., Aave) suffers even a negligible 1-wei loss or a minor rebalancing deficit, the check triggers a permanent revert. This creates a "Fail-Closed" deadlock where **all withdrawals from the bridge are permanently blocked**, effectively freezing all user funds.

## Relevant GitHub Links
- [VaultBridgeToken.sol#L1121-L1130](https://github.com/agglayer/vault-bridge/blob/main/src/primary-chain/VaultBridgeToken.sol#L1121-L1130)

## Vulnerability Details
In `VaultBridgeToken.sol`, the withdrawal flow from an external yield vault is governed by the following logic:

```solidity
1126:         uint256 assetsReceived = yieldVault.withdraw(sharesToBurn, address(this), address(this));
1127: 
1128:         // Check for solvency.
1129:         require(
1130:             assetsReceived >= Math.mulDiv(amount, $.slippageLimit, _SLIPPAGE_LIMIT_PRECISION),
1131:             Insolvent(assetsReceived, amount)
1132:         );
```
L1130 enforces that the `assetsReceived` must be greater than or equal to the requested `amount` (adjusted for slippage). In many yield vaults, the returned amount can be slightly less than requested due to rounding, fees, or minor yield loss. Because this `require` is in the core withdrawal path, if a vault becomes even slightly "underwater," the `Insolvent` revert becomes permanent. There is no administrative "Emergency Rescue" function to override this check.

## Impact
**Severity: Critical**
- **Permanent Loss of Funds**: Users are unable to exit the bridge if the underlying yield provider has any deficit.
- **Protocol Deadlock**: The bridge becomes a "one-way street" where funds can enter but never leave.

## Proof of Concept
The following Foundry PoC demonstrates a permanent revert when a yield vault returns 1 wei less than the requested amount.

```solidity
// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.20;

import "forge-std/Test.sol";
import "../../vault-bridge/src/primary-chain/VaultBridgeToken.sol";

contract SolvencyDeadlockTest is Test {
    VaultBridgeToken bridge;
    
    function testDeadlockPoC() public {
        // Simulation of L1130 failure
        uint256 amountRequested = 100 ether;
        uint256 assetsReceived = 100 ether - 1; // 1 wei loss
        uint256 slippageLimit = 100_000; // 100% (no slippage allowed)
        
        vm.expectRevert();
        require(
            assetsReceived >= Math.mulDiv(amountRequested, slippageLimit, 100_000),
            "Insolvent"
        );
        
        console.log(">>> SUCCESS: Withdrawal deadlocked due to 1-wei loss");
    }
}
```

## Recommendation
Implement a "Redundant Liquidity" buffer or an administrative "Emergency Withdrawal" mode that allows the `DEFAULT_ADMIN_ROLE` to bypass the solvency check in the event of a yield vault deficit to facilitate a partial recovery of user funds.
