# InvariantZero
### Stealth Security Researcher & Protocol Engineer

> **Consensus-level state machine auditing. Low-level EVM arithmetic security. Custom fuzzing infrastructure.**

---

## 🛡️ Protocol Remediations & Findings

### [PR #20504] Optimism (op-revm): Bedrock Deposit Nonce Inconsistency
**Status:** Open / Under Review by @rakita
- Identified a consensus-critical bug where `Create` deposits in the Bedrock hardfork failed to increment nonces on execution halts.
- Engineered a hardfork-agnostic fix in the `op-revm` handler and contributed a verified regression test suite.

### Morpho Blue: Systematic Protocol Fee Denial
**Status:** Remediated
- Discovered a systemic rounding-down vulnerability in interest accrual logic that allowed users to zero-out protocol fees.
- Implemented a `wMulUp` arithmetic patch to enforce fee-favorable rounding and preserve protocol revenue.

---

## 🛠️ Infrastructure & Tools

### `deterministic-evm-fuzzer`
**Core Logic:** Differential state-root fuzzing for L2 handlers.
- Built a high-performance Rust tool for identifying state-machine divergences between `revm` and L2-specific extensions (Optimism/Base).

---

## 🧠 Technical Stack
- **Languages:** Rust (Core), Solidity, TypeScript.
- **Deep Skills:** EVM Opcode Analysis, State-Journaling, Consensus Rules, Fixed-point Math Security.

---

*"Code is the law, but the law has bugs. I find them."*

## Portfolio Contents

*   📄 **`resume.md`**: Professional background and comprehensive summary of findings.
*   ⚙️ **`deterministic-evm-fuzzer/`**: The core of my proprietary fuzzing engine, designed to find edge-case invariant violations mathematically.
*   🔍 **`audits/`**: 
    *   **Morpho**: Fee Denial via Precision Truncation & Oracle Staleness.
    *   **Agglayer**: Solvency Deadlocks & Yield Inflation.
    *   **Basenames & BitGo**: Cross-Chain Signature Replay & Cryptographic Implementation Flaws.

## Contact
I am currently transitioning from independent bug bounty research to dedicated private consulting and protocol auditing.

*   **Twitter:** [@emokprime](https://twitter.com/emokprime)
*   **Email:** emokprime@gmail.com
*   **GitHub:** [emokprime-lang](https://github.com/emokprime-lang)
