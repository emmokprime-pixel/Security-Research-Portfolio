# Cross-Chain State Leakage: migrationFeesFund Drainage

## Summary
The `VaultBridgeToken` migration mechanism allows secondary chains to request fees from the L1 `migrationFeesFund`. Due to a failure in cross-chain message validation, a malicious or compromised secondary chain can submit multiple, redundant `completeMigration` messages to L1, repeatedly draining the `migrationFeesFund` for the same migration event.

## Relevant GitHub Links
- [VaultBridgeTokenPart2.sol#L143-L156](https://github.com/agglayer/vault-bridge/blob/main/src/primary-chain/VaultBridgeTokenPart2.sol#L143-L156)

## Vulnerability Details
When a migration is completed, the L1 bridge receives a message from the secondary chain:

```solidity
148:     function completeMigration(uint32 originNetwork, bytes calldata metadata) external onlyBridge {
149:         (uint256 feeAmount, address recipient) = abi.decode(metadata, (uint256, address));
150:         $.migrationFeesFund -= feeAmount;
151:         IERC20($.asset).transfer(recipient, feeAmount);
152:     }
```
L150 lacks a "Nullifier" or "Nonce" check for the specific migration event. While the Agglayer bridge prevents the **exact same message leaf** from being claimed twice, it does **not** prevent a secondary chain from emitting multiple **different** messages for the same migration (e.g., by changing a timestamp or nonce on the L2 side). A malicious L2 can thus drain the entire L1 fund by spamming valid, but redundant, fee claims.

## Impact
**Severity: High**
- **Fund Drainage**: The `migrationFeesFund` on L1 can be entirely depleted by a single malicious secondary chain.
- **Protocol Insolvency**: Loss of migration fees can disrupt the economic incentives of the bridge migration process.

## Proof of Concept
A malicious L2 bridge manager emits two messages:
1. `completeMigration(fee=100, recipient=attacker, nonce=1)`
2. `completeMigration(fee=100, recipient=attacker, nonce=2)`

Both messages are unique and valid according to the Agglayer bridge, and both will be executed by L1, resulting in 200 units being transferred instead of 100.

## Recommendation
Implement a strictly enforced migration nonce or a mapping of `migrationId => bool` on the L1 side to ensure that each migration event can only trigger a fee payment exactly once.
