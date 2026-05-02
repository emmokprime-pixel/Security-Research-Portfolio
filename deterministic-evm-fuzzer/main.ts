interface Finding {
    id: string;
    title: string;
    severity: 'Critical' | 'High' | 'Medium';
    target: string;
    status: string;
    description: string;
}

const findings: Finding[] = [
    {
        id: 'CRITICAL-01',
        title: 'Atomic Oracle Manipulation via ERC4626 Donation',
        severity: 'Critical',
        target: 'MorphoChainlinkOracleV2.sol',
        status: 'SUBMITTED',
        description: 'Attackers can manipulate oracle prices atomically by donating assets to the underlying ERC4626 collateral vault, leading to under-collateralized borrows.'
    },
    {
        id: 'HIGH-01',
        title: 'MetaMorpho V1 Share Inflation Attack',
        severity: 'High',
        target: 'MetaMorpho.sol',
        status: 'VERIFIED',
        description: 'First-depositor inflation attack allows malicious actors to steal funds from subsequent depositors by manipulating the initial exchange rate.'
    },
    {
        id: 'HIGH-02',
        title: 'VaultV2 Ratchet Effect Desynchronization',
        severity: 'High',
        target: 'VaultV2.sol',
        status: 'ANALYZING',
        description: 'Desynchronization between the ratchet rate and actual fee accrual leads to systemic mispricing of vault shares over long durations.'
    },
    {
        id: 'MEDIUM-01',
        title: 'Liquidation Callback Gas Griefing',
        severity: 'Medium',
        target: 'MorphoBlue.sol',
        status: 'MITIGATED',
        description: 'Malicious liquidators can grief other liquidators by consuming excessive gas in the callback, causing competing transactions to fail.'
    }
];

function renderFindings() {
    const container = document.getElementById('findings-list');
    if (!container) return;

    container.innerHTML = findings.map((finding, index) => `
        <div class="finding-card animate-in" style="animation-delay: ${0.4 + index * 0.1}s">
            <div class="finding-header">
                <div>
                    <span class="severity-badge severity-${finding.severity.toLowerCase()}">${finding.severity}</span>
                    <span style="color: var(--text-secondary); margin-left: 0.5rem; font-family: monospace;">${finding.id}</span>
                </div>
                <div style="color: var(--success); font-size: 0.75rem; font-weight: 600;">${finding.status}</div>
            </div>
            <h3 class="finding-title">${finding.title}</h3>
            <div class="finding-meta">
                <span>Target: <code>${finding.target}</code></span>
            </div>
            <p style="color: var(--text-secondary); margin-top: 1rem; font-size: 0.875rem; line-height: 1.5;">
                ${finding.description}
            </p>
        </div>
    `).join('');
}

document.addEventListener('DOMContentLoaded', () => {
    renderFindings();
});
