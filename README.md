# Security Research Portfolio | InvariantZero

> **"Silent execution, public validation."**
> 
> *While this repository and my public profile were formally established in 2026, the research contained here is the culmination of years of deep, adversarial security work. After spending years tearing down enterprise Web2 infrastructure (disclosing 150+ high-severity vulnerabilities) and quietly building proprietary EVM fuzzing architecture in the background, I am finally making my Web3 research public.*

This repository contains my public vulnerability research, deep-dive architectural audits, and the custom fuzzing tools I use to break Tier-1 DeFi protocols. I focus exclusively on absolute-depth, low-level vulnerabilities: precision truncation, cross-chain state desynchronization, and binary/opcode-level logic flaws.

## Core Capabilities
*   **Adversarial Protocol Auditing:** Identifying complex insolvency vectors that standard static analysis misses.
*   **Custom Tooling:** Creator of a proprietary, state-aware deterministic EVM fuzzing engine.
*   **Cross-Chain Security:** Deep expertise in messaging layers, signature replay, and L2 sequencer risks.

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
