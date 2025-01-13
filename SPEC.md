spec_content = """
# Pump Science Protocol Specification

## Overview

Pump Science is a Solana-based protocol implementing an advanced bonding curve mechanism with automated liquidity management and dynamic fee structures. The protocol enables the creation and management of token markets with predictable pricing and liquidity.

## Core Components

### Bonding Curve Implementation

The protocol uses a constant product market maker (x * y = k) with the following parameters:

- Virtual Token Reserves: 1,073,000,000,000,000
- Virtual SOL Reserves: 30,000,000,000
- Initial Token Supply: 793,100,000,000,000
- Reserved for Liquidity: 206,900,000,000,000
- Total Supply: 1,000,000,000,000,000

### State Management

The bonding curve maintains several critical state variables:
- Real token reserves
- Real SOL reserves
- Virtual reserves (for price computation)
- Completion status
- Start time
- Creator address

### Dynamic Fee Structure

The protocol implements a three-phase fee structure:

1. **Launch Phase** (t < 150):
   - 99% fixed fee
   - Prevents early speculation

2. **Transition Phase** (150 ≤ t ≤ 250):
   - Linear decrease following F(t) = -0.0083 * t + 2.1626
   - Gradual reduction in fees

3. **Mature Phase** (t > 250):
   - 1% fixed fee
   - Encourages market participation

All fees are collected by the protocol multisig: `3bM4hewuZFZgNXvLWwaktXMa8YHgxsnnhaRfzxJV944P`

### Liquidity Management

Automated liquidity provision triggers at 85 SOL threshold:

1. Protocol fee extraction
2. Meteora AMM pool creation with:
   - 206,900,000 tokens
   - Remaining SOL (after fees)
3. LP token locking with protocol-controlled escrow

## Trading Mechanics

### Buy Operations
- Users can purchase tokens along the bonding curve
- Price increases with supply
- Includes dynamic fee based on phase
- Minimum output amount protection

### Sell Operations
- Users can sell tokens back to the curve
- Price determined by constant product formula
- Fees deducted from output amount
- Slippage protection via minimum receive amount

## Administrative Controls

### Curve Creation
1. Parameter initialization
2. Optional whitelist configuration
3. Launch timing settings
4. Initial purchase configuration

### Access Control
- Whitelist functionality for controlled launches
- Admin controls for parameter updates
- Multisig requirements for critical operations

## Migration Process

The migration phase occurs automatically when the bonding curve reaches completion:

1. Mint remaining 206,900,000 tokens
2. Extract protocol fees
3. Create Meteora AMM pool via CPI
4. Lock liquidity with protocol-controlled escrow

## Security Considerations

1. **Access Control**
   - Role-based permissions
   - Multisig requirements for critical operations
   - Whitelist capabilities

2. **Economic Security**
   - Dynamic fee structure prevents manipulation
   - Virtual reserves maintain price stability
   - Automated liquidity management

3. **Technical Controls**
   - Program-owned PDAs for secure state management
   - Cross-Program Invocation (CPI) validation
   - Comprehensive testing suite

## Testing Infrastructure

The protocol includes two testing layers:

1. **Unit Tests** (Rust):
   - Bonding curve mathematics
   - State transitions
   - Fee calculations

2. **Integration Tests** (Bankrun):
   - End-to-end trading scenarios
   - Liquidity provision
   - Migration process