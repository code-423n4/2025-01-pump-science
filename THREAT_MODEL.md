# Threat Model

## Assets at Risk

1. User Funds
- SOL tokens used to purchase along bonding curve
- $DRUG tokens minted through the protocol
- LP tokens locked in Meteora pools

2. Protocol Infrastructure
- Bonding curve parameters and state
- Meteora liquidity pools
- Protocol fee collection mechanism

## Attack Vectors & Mitigations

### 1. Bonding Curve Manipulation

**Threat**: Attackers could attempt to manipulate the bonding curve pricing through rapid trades or large volume transactions.

**Mitigations**:
- Dynamic fee structure that starts at 99% and gradually decreases
- Fixed parameters for virtual reserves (1,073,000,000,000,000 tokens, 30,000,000,000 SOL)
- Automated liquidity management triggers at 85 SOL threshold

### 2. Liquidity Pool Vulnerabilities 

**Threat**: Attacks targeting the Meteora AMM integration during migration phase.

**Mitigations**:
- Locked LP tokens with protocol multisig authority
- Automated migration process with fixed parameters
- CPI (Cross-Program Invocation) security checks

### 3. Administrative Privilege Abuse

**Threat**: Malicious actions by privileged roles.

**Mitigations**:
- Multi-signature wallet requirement for protocol fees: `3bM4hewuZFZgNXvLWwaktXMa8YHgxsnnhaRfzxJV944P`
- Separated administrative roles with specific permissions:
  - Curve Creator: Initial setup only
  - Admin: Parameter modifications
  - Fee Recipients: Limited to fee collection


### 5. Front-running Attacks

**Threat**: MEV attacks targeting bonding curve purchases or liquidity provision.

**Mitigations**:
- Minimum output amount parameters for trades
- Time-based fee structure reducing profitability of quick trades

### 6. Migration Process Exploitation

**Threat**: Attacks during the critical migration from bonding curve to AMM.

**Mitigations**:
- Fixed token distribution (206,900,000 tokens)
- Automated liquidity seeding process
- Locked LP tokens with protocol-controlled escrow

## Security Controls

1. **Access Control**
- Whitelist functionality for controlled launches
- Multi-signature requirements for critical operations
- Role-based permissions system

2. **Economic Security**
- Dynamic fee structure
- Virtual reserves maintaining price stability
- Automated liquidity management

3. **Technical Controls**
- Program-owned PDAs for secure state management
- Cross-Program Invocation (CPI) validation
- Comprehensive testing suite:
  - Unit tests in Rust
  - End-to-end tests via Bankrun

4. **Monitoring & Recovery**
- Protocol multisig can claim trading fees
- Admin controls for parameter adjustments
- Migration state validation

## Residual Risks

1. Smart contract bugs in complex mathematical operations
2. Precision loss in bonding curve calculations
3. Potential MEV exposure during high-volume trading
4. Dependencies on external protocols (Meteora)
