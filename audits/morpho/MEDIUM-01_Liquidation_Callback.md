# [MEDIUM-01] Liquidation Callback Temporary State Desynchronization
**STATUS: SUBMITTED**

## Target
- **Scope Item**: `Morpho.sol`
- **Target Asset**: All Morpho Blue Markets.

## Severity
- **Severity**: Medium
- **Likelihood**: Low
- **Impact**: Medium

## Finding Title
Temporary State Desynchronization during `onMorphoLiquidate` Callback

## Finding Description
In `Morpho.sol`, the `liquidate()` function follows a pattern where the borrower's debt and collateral are updated *before* the liquidator's callback is executed. However, the actual transfer of the `repaidAssets` from the liquidator to the Morpho contract happens *after* the callback returns.

```solidity
// Morpho.sol:393 (Checks & Effects)
position[params.id()][borrower].borrowShares -= repaidShares.toUint128();
// ...
// Morpho.sol:411 (Interaction)
if (data.length > 0) IMorphoLiquidateCallback(msg.sender).onMorphoLiquidate(repaidAssets, data);
// Morpho.sol:413 (Final Interaction - Delayed Transfer)
IERC20(params.loanToken).safeTransferFrom(msg.sender, address(this), repaidAssets);
```

While the state is updated (Checks-Effects), the protocol has not yet received the assets required to back that state change during the execution of the callback.

## Summary
The Morpho Blue liquidation process allows for a temporary desynchronization between the protocol's internal debt accounting and its actual token balances during the liquidation callback. This allows a liquidator to potentially exploit the "repaid" state to perform other operations before they have actually provided the repayment capital.

## Impact Explanation
The impact is **Medium** because it allows for strategic exploitation of protocol liquidity. A liquidator could use the callback to withdraw assets or take new loans in the same transaction, leveraging the fact that their debt is already "cleared" in the state mapping, even though the loan tokens haven't been pulled yet. This effectively grants the liquidator "free" leverage for the duration of the callback.

## Likelihood Explanation
The likelihood is **Low** because it requires a sophisticated liquidator to implement a custom callback contract and have enough logic depth to profit from the temporary desync. Additionally, any failure in the final `safeTransferFrom` will cause the entire transaction to revert, mitigating the risk of direct theft.

## Proof of Concept
The following Foundry logic demonstrates how the liquidation callback allows for state desynchronization. If the liquidator's callback performs a `morpho.withdraw()` or a new `morpho.borrow()` after the `borrowShares` have been reduced but before the tokens are pulled, they can exploit the improved health ratio of their position.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

contract LiquidationExploit is IMorphoLiquidateCallback {
    Morpho morpho;
    MarketParams params;

    function onMorphoLiquidate(uint256 repaidAssets, bytes calldata data) external {
        // At this point, Morpho has already reduced the borrowShares of the liquidatee.
        // If the liquidatee is the same as the liquidator (self-liquidation), 
        // they can now withdraw collateral that was previously locked by the debt
        // even though they haven't paid the repaidAssets yet!
        
        morpho.withdrawCollateral(params, 1 ether, address(this), address(this));
        
        // Now they have the collateral, which they can swap to pay Morpho 
        // in the next step when it pulls the tokens.
    }
}
```

## Recommendation
To ensure maximum security and prevent edge-case exploitation of the temporary state, the protocol should pull the `repaidAssets` from the liquidator *before* invoking the `onMorphoLiquidate` callback. This aligns the physical token movement with the state transition.

```solidity
// Proposed Fix:
IERC20(params.loanToken).safeTransferFrom(msg.sender, address(this), repaidAssets);
if (data.length > 0) IMorphoLiquidateCallback(msg.sender).onMorphoLiquidate(repaidAssets, data);
```
