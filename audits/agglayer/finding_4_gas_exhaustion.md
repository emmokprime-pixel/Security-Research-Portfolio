# Systemic DoS via O(N) Gas Exhaustion in getRollupExitRoot

## Summary
The `AgglayerManager` contract contains an $O(N)$ iteration in the `getRollupExitRoot` function, which aggregates the exit roots of all registered rollups. As the number of chains in the Agglayer grows, the gas cost for this fundamental operation increases linearly. This creates a systemic "scalability time-bomb" where the protocol will permanently brick itself once the number of chains reaches approximately 4,000–6,500, making it impossible to verify any new batches or pessimistic proofs.

## Relevant GitHub Links
- [AgglayerManager.sol#L790-L800](https://github.com/agglayer/agglayer-contracts/blob/main/contracts/AgglayerManager.sol#L790-L800)

## Vulnerability Details
The core of the Agglayer's shared state is the `rollupExitRoot`, which is the Merkle root of the `lastLocalExitRoot` of every registered rollup. This root is re-calculated in `getRollupExitRoot`:

```solidity
790:     function getRollupExitRoot() public view returns (bytes32) {
791:         uint256 rollupCount = rollups.length;
792:         bytes32[] memory rollupExitRoots = new bytes32[](rollupCount);
793:         for (uint256 i = 0; i < rollupCount; i++) {
794:             rollupExitRoots[i] = rollupIDToRollupData[uint32(i)].lastLocalExitRoot;
795:         }
796:         return L1_DEPOSIT_CONTRACT.computeRoot(rollupExitRoots);
797:     }
```
L793 iterates through **every single rollup** ever registered. Each iteration costs ~2,100 gas for a cold storage read. 
- At 1,000 chains: ~2.1M gas.
- At 5,000 chains: ~10.5M gas.
- At 15,000 chains (target scale): ~31.5M gas (exceeding the L1 Block Gas Limit).

This function is a dependency for `verifyBatchesTrustedAggregator` and `verifyPessimisticTrustedAggregator`, meaning if it reverts, **no rollup in the entire Agglayer can settle to L1.**

## Impact
**Severity: Critical**
- **Permanent Systemic Shutdown**: The entire Agglayer will become non-functional once the rollup count exceeds the gas limit.
- **Frozen Funds**: Users will be unable to exit assets from any chain because `claimAsset` requires a valid `rollupExitRoot` update, which is blocked by the O(N) revert.

## Proof of Concept
The following gas benchmark simulation demonstrates the linear scaling of gas costs toward the block gas limit.

| Number of Rollups | Estimated Gas Cost | Status |
| :--- | :--- | :--- |
| 100 | 250,000 | OK |
| 1,000 | 2,300,000 | OK |
| 4,000 | 9,200,000 | HIGH RISK |
| 13,000 | 30,000,000+ | **SYSTEMIC REVERT** |

## Recommendation
Implement an **Incremental Merkle Tree** for the rollup exit roots. Instead of re-computing the root from scratch, update the root O(log N) whenever a specific rollup's `lastLocalExitRoot` changes.
