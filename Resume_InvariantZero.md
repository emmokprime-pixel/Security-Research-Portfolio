# InvariantZero
**Protocol Engineer & Stealth Security Researcher**
[GitHub: InvariantZero](https://github.com/emmokprime-pixel) | [Portfolio](https://github.com/emmokprime-pixel/Security-Research-Portfolio)

---

## 🛡️ Summary
Senior Protocol Engineer specializing in the security of low-level EVM execution environments and decentralized arithmetic. Proven track record of identifying consensus-critical vulnerabilities in Tier-1 infrastructure (Optimism) and complex economic logic flaws in major DeFi protocols (Morpho Blue). Expert in Rust, Solidity, and custom fuzzing architecture.

---

## 🏆 Key Contributions & Achievements

### **Optimism (op-revm) | Core Contributor**
*   **Vulnerability:** Identified a consensus-critical state inconsistency in the Bedrock hardfork handler where `Create` deposits failed to increment nonces on execution halts.
*   **Impact:** Remedied a potential state-root divergence vector across OP Stack chains.
*   **Remediation:** Engineered a hardfork-agnostic fix in the `op-revm` Rust crate and contributed a verified regression test suite (PR #20504).

### **Morpho Blue | Security Researcher**
*   **Vulnerability:** Discovered a systematic "Protocol Fee Denial" vulnerability caused by precision truncation in interest accrual logic.
*   **Remediation:** Developed a high-precision `wMulUp` arithmetic primitive and integrated it into the singleton core to enforce fee-favorable rounding and reclaim lost protocol revenue.
*   **Validation:** Built a deterministic Foundry PoC demonstrating 100% exploitability in zero-interest environments.

---

## 🛠️ Infrastructure & Open Source

### **`deterministic-evm-fuzzer` (Lead Architect)**
*   Developed a high-performance Rust framework for differential state-root fuzzing.
*   Engineered state-journaling hooks to identify subtle logic divergences between the base `revm` and L2-specific extensions (Optimism/Base/Arbitrum).
*   Integrated interest-rate-model (IRM) divergence testing for high-precision lending protocols.

---

## 🧪 Technical Skills
*   **Languages:** Rust (Expert), Solidity (Advanced), TypeScript, Python.
*   **EVM Internals:** Opcode-level auditing, State transitions, Memory/Stack management, Precompile analysis.
*   **Security:** Fuzzing (Foundry, Custom Rust tools), Formal Verification (foundational), Arithmetic Safety.
*   **Systems:** Linux, Git, CI/CD (GitHub Actions/CircleCI), Distributed Systems.

---

## 📚 Education & Research
*   **Independent Research:** Formal analysis of L2 Sequencer risk and cross-chain messaging security.
*   **EVM Mechanics:** Deep-dive audit of the `reth` execution client and `revm` handler logic.

---
*"InvariantZero: Precision security for the decentralized stack."*
