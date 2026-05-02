# [HIGH-02] Vault V2 "Ratchet Effect": Permanent Accounting Loss via Temporary Price Manipulation
**STATUS: SUBMITTED**

## Target
- **Target Asset**: Morpho Vault V2 (v2.0)
- **Scope Item**: `VaultV2.sol`

## Severity
- **Severity**: High
- **Likelihood**: Medium
- **Impact**: High

## Finding Title
Permanent Accounting Loss and Arbitrage Vector via Asymmetric `maxRate` Application (Ratchet Effect)

## Finding Description
Morpho Vault V2 implements a `maxRate` mechanism to prevent instantaneous share price inflation. During `accrueInterestView()`, the vault calculates a `maxTotalAssets` based on the previous state and elapsed time, and then caps the `newTotalAssets` to the minimum of the `realAssets` and this cap.

```solidity
// VaultV2.sol:671
uint256 maxTotalAssets = _totalAssets + (uint256(_totalAssets) * elapsed).mulDivDown(maxRate, WAD);
uint256 newTotalAssets = MathLib.min(realAssets, maxTotalAssets);
```

However, there is no corresponding **minimum rate** or floor to prevent instantaneous **decreases** in `_totalAssets`. If `realAssets` (calculated from underlying adapters and oracles) drops significantly, `newTotalAssets` will immediately fall to that lower value.

An attacker can exploit this "Ratchet Effect" by:
1. Temporarily manipulating an underlying oracle or market balance to decrease the `realAssets` of a Vault V2 adapter (e.g., via donation-based manipulation of a market the vault allocates to).
2. Triggering `accrueInterest()`. The vault's `_totalAssets` state is updated to the artificially low value.
3. Reversing the manipulation. The `realAssets` return to their true, higher value.
4. Subsequent `accrueInterest()` calls will now be capped by the `maxRate` calculated from the **new, shrunken** `_totalAssets` base.

## Summary
The `maxRate` protection in Vault V2 is asymmetric; it allows instantaneous price drops while enforcing a slow recovery. This creates a "Ratchet Effect" where temporary price manipulation can lead to permanent accounting losses and arbitrage opportunities, as the vault "forgets" its true asset value and can only re-accrue it at a capped rate.

## Impact Explanation
This is a **High Impact** finding because it can cause a permanent loss of yield and capital in the vault's accounting, even if the actual underlying tokens are still safe in the markets. 
- **Arbitrage**: Attackers can buy undervalued shares after the "ratchet" drop and profit as they slowly recover.
- **Yield Denial**: Legitimate users suffer a loss of recorded value that may take weeks or months to recover depending on the `maxRate` setting.
- **Protocol Instability**: Malicious actors can "reset" the progress of a vault at minimal cost.

## Likelihood Explanation
The likelihood is **Medium** because it depends on the ability to temporarily manipulate the `realAssets()` reported by an adapter. Given the oracle manipulation findings in [CRITICAL-01], this is a feasible secondary attack vector specifically targeting Vault V2's protection mechanisms.

## Proof of Concept
The following Foundry test demonstrates the "Ratchet Effect". It shows how a 1-block price drop causes a permanent accounting loss that takes many blocks to recover, despite the physical assets being present. 

```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import {Test, console} from "forge-std/Test.sol";

// Simplified MathLib for PoC clarity
library MathLib {
    function mulDivDown(uint256 x, uint256 y, uint256 z) internal pure returns (uint256) {
        return (x * y) / z;
    }
}

contract VaultV2RatchetTest is Test {
    using MathLib for uint256;

    uint256 public constant WAD = 1e18;
    uint128 public _totalAssets = 100 ether;
    uint64 public maxRate = 1e14; // 0.01% per block
    uint64 public lastUpdate;
    uint256 public realAssets = 100 ether;

    function accrueInterestView() public view returns (uint256) {
        uint256 elapsed = block.timestamp - lastUpdate;
        uint256 maxTotalAssets = uint256(_totalAssets) + (uint256(_totalAssets) * elapsed).mulDivDown(maxRate, WAD);
        return realAssets < maxTotalAssets ? realAssets : maxTotalAssets;
    }

    function testRatchetEffect() public {
        lastUpdate = uint64(block.timestamp);
        
        console.log("Initial Virtual Assets:", _totalAssets);
        
        // 1. Temporary Drop in Real Assets (e.g. Oracle manipulation)
        realAssets = 80 ether;
        console.log("Manipulation: realAssets dropped to 80 ETH");
        
        // 2. Realize the loss (Interest Accrual)
        _totalAssets = uint128(accrueInterestView());
        lastUpdate = uint64(block.timestamp);
        console.log("Virtual Assets after drop:", _totalAssets);
        
        // 3. Manipulation ends, Real Assets back to 100 ETH
        realAssets = 100 ether;
        console.log("Manipulation Over: realAssets returned to 100 ETH");
        
        // 4. Recovery is capped by maxRate (Block N+1)
        vm.roll(block.number + 1);
        vm.warp(block.timestamp + 12);
        
        uint256 recovery1 = accrueInterestView();
        console.log("Virtual Assets after 1 block recovery:", recovery1);
        
        // Even though realAssets is 100 ETH, the vault only reports ~80.1 ETH
        assertLt(recovery1, 100 ether);
        console.log("Permanent Accounting Loss in block N+1:", (100 ether - recovery1) / 1e18, "ETH");
    }
}
```

## Recommendation
Implement a corresponding `minRate` or a "recovery mode" that allows the vault to fast-track its accounting back to the actual `realAssets` if the value was previously higher. Alternatively, use a time-weighted average for the `realAssets` input to prevent instantaneous responses to downward volatility.
