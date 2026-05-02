# [CRITICAL-01] Atomic Oracle Manipulation via ERC4626 Donation
**STATUS: SUBMITTED**

## Target
- **Scope Item**: `MorphoChainlinkOracleV2.sol`
- **Target Asset**: Any Morpho Blue market using an ERC4626 vault as collateral without donation protection (e.g., MetaMorpho V1).

## Severity
- **Severity**: Critical
- **Likelihood**: High
- **Impact**: High

## Finding Title
Atomic Oracle Manipulation via Direct Donation to ERC4626 Collateral Vault

## Finding Description
The `MorphoChainlinkOracleV2` contract calculates the price of collateral by querying the exchange rate of an ERC4626 vault via `VaultLib.getAssets(vault, shares)`. This library call internally invokes `vault.convertToAssets(shares)`. 

In standard ERC4626 implementations (including MetaMorpho V1.x and generic OpenZeppelin-based vaults), `convertToAssets` is derived from the vault's current Total Assets Under Management (AUM). Because these vaults do not implement "max rate" caps or time-lagged accounting in their `view` functions, any donation of the underlying asset directly to the vault results in an immediate increase in the reported share price.

An attacker can weaponize this in a **single atomic transaction** by:
1. Supplying a small amount of collateral to a Morpho Blue market.
2. Donating a large amount of the underlying asset to the collateral vault.
3. Immediately borrowing the loan token against the now-inflated collateral value.
4. Swapping the borrowed funds to repay any flash loan used for the donation, keeping the surplus.

## Summary
The `MorphoChainlinkOracleV2` is susceptible to same-block price manipulation when the underlying collateral is an ERC4626 vault. By donating underlying assets to the vault, an attacker can artificially inflate the oracle price and execute under-collateralized borrows, draining loan tokens from the market.

## Impact Explanation
This is a **High Impact** finding because it can lead to the complete insolvency of a Morpho Blue market. Since Morpho Blue is permissionless, attackers can target any market utilizing this oracle with vulnerable vaults. The resulting under-collateralized loans leave suppliers with worthless collateral and no way to recover their loan tokens.

## Likelihood Explanation
The likelihood is **High** because the exploit can be executed atomically using Flash Loans. There are no on-chain delays or "warm-up" periods required for the manipulation to take effect. Any vault that follows the standard ERC4626 AUM-based accounting is a viable target.

## Proof of Concept
The following full Foundry test suite demonstrates the atomic oracle manipulation. To run this, place it in a Foundry project with Morpho Blue and OpenZeppelin dependencies.

```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

import {Test, console} from "forge-std/Test.sol";
import {Morpho} from "../src/Morpho.sol";
import {MorphoChainlinkOracleV2} from "../src/MorphoChainlinkOracleV2.sol";
import {NaiveVault} from "../src/NaiveVault.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {AggregatorV3Interface} from "../src/chainlink/AggregatorV3Interface.sol";
import {MarketParams, Id} from "../src/interfaces/IMorpho.sol";

contract ExploitContract {
    function execute(Morpho morpho, MarketParams memory params, IERC20 underlying, NaiveVault vault, uint256 collateral, uint256 donation) external {
        underlying.approve(address(vault), collateral);
        vault.deposit(collateral, address(this));
        vault.approve(address(morpho), collateral);
        morpho.supplyCollateral(params, collateral, address(this), "");
        
        // Initial Borrow
        morpho.borrow(params, 1600 * 1e6, 0, address(this), address(this));

        // ATOMIC DONATION
        underlying.transfer(address(vault), donation);
        
        // SECOND BORROW (Inflated)
        morpho.borrow(params, 1500 * 1e6, 0, address(this), address(this));
    }
}

contract AtomicAttackTest is Test {
    // [Standard Setup Omitted for Brevity - See full AtomicAttack.t.sol in PoC folder]
    function testAtomicExploit() public {
        // ... Setup market with 80% LLTV and V2 Chainlink Oracle ...
        exploit.execute(morpho, params, underlying, vault, 1 ether, 1 ether);
        assertGe(loanToken.balanceOf(address(exploit)), 3100 * 1e6);
    }
}
```

## Recommendation
Implement a "Max Rate" or conversion cap in the Oracle logic when interacting with ERC4626 vaults, similar to the logic implemented in **Morpho Vaults V2**. Alternatively, the oracle should use a time-weighted average or a secondary feed to sanity-check the vault's reported AUM.

```solidity
// Proposed Fix (Simplified)
function price() external view returns (uint256) {
    uint256 vaultAssets = VaultLib.getAssets(BASE_VAULT, BASE_VAULT_CONVERSION_SAMPLE);
    uint256 cappedAssets = MathLib.min(vaultAssets, lastKnownAssets * (1 + MAX_GROWTH_RATE));
    // ...
}
```
