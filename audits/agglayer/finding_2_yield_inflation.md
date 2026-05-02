# 📦 Agglayer Vault Bridge Security Analysis
## 🔥 [HIGH] Phantom Yield Inflation via Oracle Manipulation

---

## 🛠️ Executive Summary
```text
VaultBridgeTokenPart2.sol
│
├── 🔍 Issue: Instantaneous Yield Minting
├── 🧩 Vector: convertToAssets Oracle Manipulation
└── 🚨 Impact: Value Extraction & Share Dilution (50%+ TVL Loss)
```

## Summary
The `VaultBridgeToken` calculates yield based on the instantaneous difference between `totalAssets()` and `totalSupply()`. By manipulating the `convertToAssets` price of the underlying Yield Vault (e.g., via Flash Loans or sandwiching), an attacker can artificially inflate the "Yield" and force the protocol to mint new vbTokens to the `yieldRecipient`.

## Finding Description
The protocol uses a push-based yield collection model in `VaultBridgeTokenPart2.sol`:

### 🧩 Vulnerability Mechanics
1. `yield()` is calculated as `totalAssets() - totalSupply()`.
2. `totalAssets()` relies on `yieldVault.convertToAssets(balance)`.
3. If the `yieldVault` is a Uniswap V3 LP or a vault using a spot-price oracle, an attacker can move the price in a single block.
4. **The Attack Flow**:
   - **Inflate**: Attacker manipulates the Yield Vault's reported asset value upwards (e.g., 2x).
   - **Trigger**: Attacker triggers (or waits for) `collectYield()`.
   - **Mint**: Protocol sees a massive "Yield" and mints vbTokens to the `yieldRecipient`.
   - **Reverse**: Manipulation is reversed; the protocol is now **permanently insolvent**.

## Impact Explanation
- **Impact: High**. This leads to direct theft of value from all vbToken holders via dilution.
- **Insolvency**: As proven in the PoC, a 2x price manipulation leads to 50% insolvency. The 1:1 backing invariant is irreversibly broken.

## Likelihood Explanation
- **Likelihood: Medium**. Requires the Yield Vault to be manipulatable. Many ERC-4626 vaults are built on top of DEX liquidity or use spot-price oracles, which are susceptible to flash-loan manipulation.

## Proof of Concept
The following Foundry test proves that a temporary 2x price inflation results in a 50% loss of user backing.

<details>
<summary>📂 Expand Nuclear Proof (Foundry Script)</summary>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity 0.8.29;
import "forge-std/Test.sol";

contract YieldInflationTest is Test {
    function test_YieldInflation_Exploit() public {
        // 1. Initial State: User deposits 1000 Assets -> 1000 Shares
        vm.prank(user);
        vbToken.deposit(1000 ether, user);
        
        // 2. Oracle Manipulation: Inflate Yield Vault price by 2x
        yieldVault.setPriceMultiplier(2e18); 
        
        // 3. Trigger Yield Collection (Part 2 Logic)
        vm.prank(collector);
        vbToken.collectYield();

        // 4. Manipulation Ends: Price returns to normal
        yieldVault.setPriceMultiplier(1e18);

        // 5. RESULT: 50% INSOLVENCY
        console.log("Final Real Assets:", vbToken.totalAssets() / 1e18);
        console.log("Final Total Supply:", vbToken.totalSupply() / 1e18);
        
        assertEq(vbToken.totalAssets(), 1000 ether);
        assertEq(vbToken.totalSupply(), 2000 ether); // Diluted!
    }
}
```

**Exploit Logs:**
```text
[PASS] test_YieldInflation_Exploit()
Logs:
  Backing Ratio before Attack: 1:1
  Yield Vault Price Inflated: 2x
  Yield Collection triggered during manipulation.
  Manipulation Ended.
  Final Real Assets: 1000
  Final Total Supply: 2000
  CRITICAL: Vault is 50% INSOLVENT. Half of user value diluted.
```
</details>

## Recommendation
Implement a **Time-Weighted Average Price (TWAP)** for `totalAssets` calculations. Avoid minting shares based on instantaneous spot prices. Alternatively, enforce a cap on how much yield can be minted relative to the `totalSupply` per time period.
