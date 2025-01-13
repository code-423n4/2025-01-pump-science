# Pump Science audit details
- Total Prize Pool: $20,000 in USDC
  - HM awards: $15,900 in USDC
  - QA awards: $700 in USDC 
  - Judge awards: $1,700 in USDC
  - Validator awards: $1,200 in USDC 
  - Scout awards: $500 in USDC
- [Read our guidelines for more details](https://docs.code4rena.com/roles/wardens)
- Starts January 15, 2025 20:00 UTC
- Ends January 21, 2025 20:00 UTC

**Note re: risk level upgrades/downgrades**

Two important notes about judging phase risk adjustments: 
- High- or Medium-risk submissions downgraded to Low-risk (QA)) will be ineligible for awards.
- Upgrading a Low-risk finding from a QA report to a Medium- or High-risk finding is not supported.

As such, wardens are encouraged to select the appropriate risk level carefully during the submission phase.

## Automated Findings / Publicly Known Issues



_Note for C4 wardens: Anything included in this `Automated Findings / Publicly Known Issues` section is considered a publicly known issue and is ineligible for awards._

Some accounts are not being checked in the migration instructions. This is fine because those accounts are being checked by the Meteora program itself


# Overview

Pump Science Bonding Curve Protocol is a Solana protocol implementing an advanced bonding curve mechanism for fundraising and sustainable project funding. This protocol enables compound submitters to launch their own token ($DRUG) with dynamic fee structures and automated liquidity management.

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


## Links

- **Previous audits:**  Not completed yet
- **Documentation:** https://github.com/moleculeprotocol/pump-science-contract (Jay to double check) ✅
- **Website:** https://www.pump.science/
- **X/Twitter:** https://x.com/pumpdotscience
- **Telegram:** https://t.me/pump_science

---


# Scope

*See [scope.txt](https://github.com/code-423n4/2025-01-pump-science/blob/main/scope.txt)*

### Files in scope


| File                                                                   | SLOC     |
|------------------------------------------------------------------------|----------|
| ./programs/pump-science/src/instructions/curve/swap.rs                 | 343      |
| ./programs/pump-science/src/instructions/migration/create_pool.rs      | 323      |
| ./programs/pump-science/src/state/bonding_curve/curve.rs               | 289      |
| ./programs/pump-science/src/state/bonding_curve/tests.rs               | 259      |
| ./programs/pump-science/src/instructions/curve/create_bonding_curve.rs | 196      |
| ./programs/pump-science/src/instructions/migration/lock_pool.rs        | 184      |
| ./programs/pump-science/src/state/global.rs                            | 141      |
| ./programs/pump-science/src/state/bonding_curve/locker.rs              | 96       |
| ./programs/pump-science/src/errors.rs                                  | 74       |
| ./programs/pump-science/src/events.rs                                  | 61       |
| ./programs/pump-science/src/instructions/admin/set_params.rs           | 47       |
| ./programs/pump-science/src/lib.rs                                     | 47       |
| ./programs/pump-science/src/state/meteora.rs                           | 44       |
| ./programs/pump-science/src/instructions/admin/add_wl.rs               | 37       |
| ./programs/pump-science/src/instructions/admin/initialize.rs           | 37       |
| ./programs/pump-science/src/state/bonding_curve/structs.rs             | 33       |
| ./programs/pump-science/src/instructions/admin/remove_wl.rs            | 28       |
| ./programs/pump-science/src/state/whitelist.rs                         | 9        |
| ./programs/pump-science/src/util.rs                                    | 9        |
| ./programs/pump-science/src/constants.rs                               | 6        |
| ./programs/pump-science/src/instructions/mod.rs                        | 6        |
| ./programs/pump-science/src/state/bonding_curve/mod.rs                 | 5        |
| ./programs/pump-science/src/instructions/admin/mod.rs                  | 4        |
| ./programs/pump-science/src/instructions/migration/mod.rs              | 4        |
| ./programs/pump-science/src/state/mod.rs                               | 4        |
| ./programs/pump-science/src/instructions/curve/mod.rs                  | 3        |
| **SUM:**                                                               | **2289** |

### Files out of scope

*See [out_of_scope.txt](https://github.com/code-423n4/2025-01-pump-science/blob/main/out_of_scope.txt)*



## Scoping Q &amp; A

### General questions

| Question                                | Answer                       |
| --------------------------------------- | ---------------------------- |
| ERC20 used by the protocol              |       None             |
| Test coverage                           | N/A                          |
| ERC721 used  by the protocol            |            None              |
| ERC777 used by the protocol             |           None                |
| ERC1155 used by the protocol            |              None            |
| Chains the protocol will be deployed on | Solana  |


### External integrations (e.g., Uniswap) behavior in scope:


| Question                                                  | Answer |
| --------------------------------------------------------- | ------ |
| Enabling/disabling fees (e.g. Blur disables/enables fees) | No   |
| Pausability (e.g. Uniswap pool gets paused)               |  No   |
| Upgradeability (e.g. Uniswap gets upgraded)               |   No  |


### EIP compliance checklist
N/A



# Additional context

## Main invariants

1. If whitelist is enabled only whitelisted users should be able to create a bonding curve, otherwise everyone is allowed
2. Only the global authority can update parameters of the configuration account
3. Migration can only happen when complete=true which happens when all available tokens are sold out (793,100,000) and approx. 85 SOL is raised
4. Only the migration authority can execute migration instructions


## Attack ideas (where to focus for bugs)
N/A


## All trusted roles in the protocol

| Role                                | Description                       |
| --------------------------------------- | ---------------------------- |
| Admin Authority                          | can make changes to configuration account                |
| Migration Authority                             | can call the migration instructions                       |
| Whitelist members (optional feature)  |  creating bonding curves  |

## Describe any novel or unique curve logic or mathematical models implemented in the contracts:

The protocol implements a constant product bonding curve (x * y = k) with the following initial parameters:

- Initial Virtual Token Reserves: 1,073,000,000,000,000
- Initial Virtual SOL Reserves: 30,000,000,000
- Initial Real Token Reserves: 793,100,000,000,000
- Total Token Supply: 1,000,000,000,000,000

The bonding curve ensures price discovery and continuous liquidity for the token.


## Running tests




```bash
git clone https://github.com/code-423n4/2025-01-pump-science.git
cd 2025-01-pump-science
pnpm install
pnpm programs:build
pnpm programs:test # unit tests of smart contract specific bonding curve functions
pnpm test:bankrun  # integration test suite
```




## Miscellaneous
Employees of Pump Science and employees' family members are ineligible to participate in this audit.

Code4rena's rules cannot be overridden by the contents of this README. In case of doubt, please check with C4 staff.
