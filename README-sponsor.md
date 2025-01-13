# Pump Science Bonding Curve Protocol

A Solana protocol implementing an advanced bonding curve mechanism for fundraising and sustainable project funding. This protocol enables compound submitters to launch their own token ($DRUG) with dynamic fee structures and automated liquidity management.

## Core Features

### Bonding Curve Mechanism

The protocol implements a constant product bonding curve (x * y = k) with the following initial parameters:

- Initial Virtual Token Reserves: 1,073,000,000,000,000
- Initial Virtual SOL Reserves: 30,000,000,000
- Initial Real Token Reserves: 793,100,000,000,000
- Total Token Supply: 1,000,000,000,000,000

The bonding curve ensures price discovery and continuous liquidity for the token.

### Dynamic Fee Structure

Fees are calculated using a piecewise linear function based on user participation slots:

1. Early Phase (t < 150):
   - Fixed 99% fee
2. Transition Phase (150 ≤ t ≤ 250):
   - Linear decrease: F(t) = -0.0083 * t + 2.1626
3. Mature Phase (t > 250):
   - Fixed 1% fee

All fees are directed to the protocol's multisig wallet: `3bM4hewuZFZgNXvLWwaktXMa8YHgxsnnhaRfzxJV944P`

### Automated Liquidity Management

When the bonding curve accumulates 85 SOL:
1. X SOL is sent to the protocol multisig
2. Remaining SOL is used to seed a Meteora constant product liquidity pool
3. LP tokens are locked with claim authority assigned to the protocol multisig

## Administrative Roles

### Curve Creator
- Can initialize new bonding curves
- Sets initial parameters and optional whitelist
- Configures launch timing and initial purchases

### Admin
- Can modify protocol parameters
- Manages fee settings
- Controls whitelist status

### Fee Recipients
- Protocol Multisig (`3bM4hewuZFZgNXvLWwaktXMa8YHgxsnnhaRfzxJV944P`)
  - Receives trading fees
  - Has authority over locked LP tokens
  - Receives swapped USDC from liquidity migrations

## Creating a Bonding Curve

To create a new bonding curve:

1. Initialize curve parameters
2. Optional: Enable whitelist
3. Set launch timing
4. Configure initial purchases

Trading is enabled along the bonding curve until 85 SOL are raised and all 793,100,000 tokens are sold.

## Migration
Migration is a critical process that occurs once the bonding curve has completed and the tokens are empty. It involves:

1. Minting the remaining 206,900,000 tokens
2. Sending the experiment fee to the multisig wallet
3. A CPI (Cross-Program Invocation) call to create a Meteora Dynamic AMM (Automated Market Maker). It uses 206,900,000 matched with 85SOL - experiment fee

4. A separate instruction CPI call that locks the liquidity in the AMM and creates an escrow from which the multisig can claim trading fees.

## Testing

The repository includes two types of tests to ensure the functionality and reliability of the Pump Science Bonding Curve Protocol:

1. **Unit Tests**: These tests are written in Rust and are located within the program's source code. They focus on testing individual components of the bonding curve logic. You can run these tests using the following command:
   ```bash
   pnpm programs:test
   ```

2. **Bankrun End-to-End Tests**: These tests simulate real-world scenarios to validate the entire protocol's behavior from start to finish. They are written in TypeScript and can be executed with the following command:
   ```bash
   pnpm test:bankrun
   ```
