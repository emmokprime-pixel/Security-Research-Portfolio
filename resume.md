# InvariantZero
**Independent Security Researcher & Protocol Auditor**
[GitHub: emokprime-lang](https://github.com/emokprime-lang) | [Twitter: @emokprime](https://twitter.com/emokprime) | emokprime@gmail.com

### Professional Summary
Security Researcher specializing in low-level EVM architecture, cross-chain invariant analysis, and AI-augmented vulnerability research. Transitioned from a high-volume Web2 bug bounty background (150+ disclosed vulnerabilities) to Web3 to focus on deterministic architectural logic flaws. Creator of a proprietary, state-aware EVM fuzzing engine. Proven track record of identifying high-severity economic logic flaws, precision truncation vulnerabilities, and cross-chain desyncs across Tier-1 DeFi protocols. 

### Core Competencies
*   **Smart Contract Security:** Solidity, Rust (Soroban), Yul/Assembly, EVM Opcodes.
*   **Vulnerability Research:** Protocol Insolvency vectors, Cross-chain State Desync, Precision/Rounding Exploits, Signature Replay, AI-Augmented Static Analysis.
*   **Tooling & Testing:** Foundry (Advanced PoC development), Deterministic State Fuzzing, Binary/Bytecode Analysis (WASM), Python.
*   **Infrastructure Security:** Web Application Firewalls (WAF), Edge network configuration, HTTP Header Smuggling.

---

### Key Security Research & Audit Findings

**Morpho Protocol | Independent Vulnerability Research**
*   **Protocol Fee Denial via Precision Truncation:** Discovered a high-severity economic exploit utilizing high-frequency micro-accruals and `wMulDown` floor-rounding to permanently deny the Morpho DAO protocol fees without requiring privileged access.
*   **Oracle Staleness & PreLiquidation Divergence:** Developed deterministic Foundry PoCs proving L2 sequencer downtime could be leveraged to execute high-impact liquidations at stale prices.

**OKX Wallet Core (EIP-7702) | Binary-Level Security Audit**
*   Conducted deep-dive bytecode and opcode-level analysis of OKX's WalletCore and Storage implementations.
*   Bypassed high-level source code to identify low-level logic flaws and architectural weaknesses in account abstraction flows.

**Polygon Agglayer & CDK | Cross-Chain Security Research**
*   Discovered critical state corruption and permanent DoS vulnerabilities via solvency check deadlocks in Core and Sovereign Bridge contracts.
*   Identified Native ETH context erasure vectors and yield inflation exploits during cross-chain message passing.

**Circle Arc (Stablecoin-XLM) | Soroban Wasm Binary Analysis**
*   Performed adversarial analysis on Rust-based Soroban smart contracts targeting economic logic flaws.
*   Conducted low-level Wasm binary reversing to build deterministic PoC tests in the Soroban environment for HackerOne submission.

**Coinbase Basenames & BitGo | Cryptographic Implementation Flaws**
*   Identified a Critical cross-chain signature replay vulnerability in Coinbase Basenames on Base L2 due to a missing `block.chainid` in the signature hashing logic.
*   Uncovered cross-chain signature replay vulnerabilities in the BitGo v2 multisig ecosystem, verifying findings with deterministic Foundry PoCs.

---

### Custom Tooling & Open Source Development

**Deterministic EVM Fuzzer (Creator/Lead Developer)**
*   Architected and developed a proprietary, state-aware deterministic fuzzing engine designed to find edge-case invariant violations that standard fuzzers miss.
*   Built to programmatically identify math truncation, state desynchronization, and fee-denial vectors across complex DeFi architectures.

**Security Research Portfolio (github.com/emokprime-lang)**
*   Maintains a repository of weaponized Foundry Proof-of-Concepts (PoCs) demonstrating complex DeFi exploits on live mainnet forks.

---

**Independent Web3 Security Researcher (InvariantZero)**
*2024 – Present*
*   Conducting adversarial research on cross-chain bridges, lending protocols, and infrastructure resulting in multiple verified high-severity findings.
*   Utilizing advanced AI models to accelerate code comprehension, isolate vulnerable logic paths, and automate complex fuzzing harnesses, resulting in a 10x increase in audit efficiency.

**Web2 Bug Bounty Hunter & Penetration Tester**
*2022 – 2024*
*   Identified and disclosed over 150 vulnerabilities across enterprise Web2 infrastructure.
*   Recognized the structural limitations and "lottery" nature of public bug bounties, pivoting completely to deep-level Web3 architectural security and private consulting.
