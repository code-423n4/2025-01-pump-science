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

The 4naly3er report can be found [here](https://github.com/code-423n4/2024-12-pump-science/blob/main/4naly3er-report.md).



_Note for C4 wardens: Anything included in this `Automated Findings / Publicly Known Issues` section is considered a publicly known issue and is ineligible for awards._

Some accounts are not being checked in the migration instructions. This is fine because those accounts are being checked by the Meteora program itself

✅ SCOUTS: Please format the response above 👆 so its not a wall of text and its readable.

# Overview

[ ⭐️ SPONSORS: add info here ]

## Links

- **Previous audits:**  Not completed yet
  - ✅ SCOUTS: If there are multiple report links, please format them in a list.
- **Documentation:** https://github.com/moleculeprotocol/pump-science-contract (Jay to double check)
- **Website:** https://www.pump.science/
- **X/Twitter:** https://x.com/pumpdotscience
- **Telegram:** https://t.me/pump_science

---

# Scope

[ ✅ SCOUTS: add scoping and technical details here ]

### Files in scope
- ✅ This should be completed using the `metrics.md` file
- ✅ Last row of the table should be Total: SLOC
- ✅ SCOUTS: Have the sponsor review and and confirm in text the details in the section titled "Scoping Q amp; A"

*For sponsors that don't use the scoping tool: list all files in scope in the table below (along with hyperlinks) -- and feel free to add notes to emphasize areas of focus.*

| Contract | SLOC | Purpose | Libraries used |  
| ----------- | ----------- | ----------- | ----------- |
| [contracts/folder/sample.sol](https://github.com/code-423n4/repo-name/blob/contracts/folder/sample.sol) | 123 | This contract does XYZ | [`@openzeppelin/*`](https://openzeppelin.com/contracts/) |

### Files out of scope
✅ SCOUTS: List files/directories out of scope

## Scoping Q &amp; A

### General questions
### Are there any ERC20's in scope?: No

✅ SCOUTS: If the answer above 👆 is "Yes", please add the tokens below 👇 to the table. Otherwise, update the column with "None".




### Are there any ERC777's in scope?: 

✅ SCOUTS: If the answer above 👆 is "Yes", please add the tokens below 👇 to the table. Otherwise, update the column with "None".



### Are there any ERC721's in scope?: No

✅ SCOUTS: If the answer above 👆 is "Yes", please add the tokens below 👇 to the table. Otherwise, update the column with "None".



### Are there any ERC1155's in scope?: No

✅ SCOUTS: If the answer above 👆 is "Yes", please add the tokens below 👇 to the table. Otherwise, update the column with "None".



✅ SCOUTS: Once done populating the table below, please remove all the Q/A data above.

| Question                                | Answer                       |
| --------------------------------------- | ---------------------------- |
| ERC20 used by the protocol              |       🖊️             |
| Test coverage                           | ✅ SCOUTS: Please populate this after running the test coverage command                          |
| ERC721 used  by the protocol            |            🖊️              |
| ERC777 used by the protocol             |           🖊️                |
| ERC1155 used by the protocol            |              🖊️            |
| Chains the protocol will be deployed on | OtherSolana  |

### ERC20 token behaviors in scope

| Question                                                                                                                                                   | Answer |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ |
| [Missing return values](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#missing-return-values)                                                      |    |
| [Fee on transfer](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#fee-on-transfer)                                                                  |   |
| [Balance changes outside of transfers](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#balance-modifications-outside-of-transfers-rebasingairdrops) |    |
| [Upgradeability](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#upgradable-tokens)                                                                 |    |
| [Flash minting](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#flash-mintable-tokens)                                                              |    |
| [Pausability](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#pausable-tokens)                                                                      |    |
| [Approval race protections](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#approval-race-protections)                                              |    |
| [Revert on approval to zero address](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-approval-to-zero-address)                            |    |
| [Revert on zero value approvals](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-zero-value-approvals)                                    |    |
| [Revert on zero value transfers](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-zero-value-transfers)                                    |    |
| [Revert on transfer to the zero address](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-transfer-to-the-zero-address)                    |    |
| [Revert on large approvals and/or transfers](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-large-approvals--transfers)                  |    |
| [Doesn't revert on failure](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#no-revert-on-failure)                                                   |    |
| [Multiple token addresses](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#revert-on-zero-value-transfers)                                          |    |
| [Low decimals ( < 6)](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#low-decimals)                                                                 |    |
| [High decimals ( > 18)](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#high-decimals)                                                              |    |
| [Blocklists](https://github.com/d-xo/weird-erc20?tab=readme-ov-file#tokens-with-blocklists)                                                                |    |

### External integrations (e.g., Uniswap) behavior in scope:


| Question                                                  | Answer |
| --------------------------------------------------------- | ------ |
| Enabling/disabling fees (e.g. Blur disables/enables fees) | No   |
| Pausability (e.g. Uniswap pool gets paused)               |  No   |
| Upgradeability (e.g. Uniswap gets upgraded)               |   No  |


### EIP compliance checklist
N/A

✅ SCOUTS: Please format the response above 👆 using the template below👇

| Question                                | Answer                       |
| --------------------------------------- | ---------------------------- |
| src/Token.sol                           | ERC20, ERC721                |
| src/NFT.sol                             | ERC721                       |


# Additional context

## Main invariants

1. If whitelist is enabled only whitelisted users should be able to create a bonding curve, otherwise everyone is allowed
2. Only the global authority can update parameters of the configuration account
3. Migration can only happen when complete=true which happens when all available tokens are sold out (793,100,000) and approx. 85 SOL is raised
4. Only the migration authority can execute migration instructions

✅ SCOUTS: Please format the response above 👆 so its not a wall of text and its readable.

## Attack ideas (where to focus for bugs)
None

✅ SCOUTS: Please format the response above 👆 so its not a wall of text and its readable.

## All trusted roles in the protocol

Admin Authority - can make changes to configuration account
Migration Authority - can call the migration instructions
Optional whitelist feature for creating bonding curves

✅ SCOUTS: Please format the response above 👆 using the template below👇

| Role                                | Description                       |
| --------------------------------------- | ---------------------------- |
| Owner                          | Has superpowers                |
| Administrator                             | Can change fees                       |

## Describe any novel or unique curve logic or mathematical models implemented in the contracts:

The protocol implements a constant product bonding curve (x * y = k) with the following initial parameters:

- Initial Virtual Token Reserves: 1,073,000,000,000,000
- Initial Virtual SOL Reserves: 30,000,000,000
- Initial Real Token Reserves: 793,100,000,000,000
- Total Token Supply: 1,000,000,000,000,000

The bonding curve ensures price discovery and continuous liquidity for the token.

✅ SCOUTS: Please format the response above 👆 so its not a wall of text and its readable.

## Running tests

pnpm install
pnpm programs:build
pnpm programs:test (unit tests of smart contract specific bonding curve functions)
pnpm test:bankrun (integration test suite)

✅ SCOUTS: Please format the response above 👆 using the template below👇

```bash
git clone https://github.com/code-423n4/2023-08-arbitrum
git submodule update --init --recursive
cd governance
foundryup
make install
make build
make sc-election-test
```
To run code coverage
```bash
make coverage
```
To run gas benchmarks
```bash
make gas
```

✅ SCOUTS: Add a screenshot of your terminal showing the gas report
✅ SCOUTS: Add a screenshot of your terminal showing the test coverage

## Miscellaneous
Employees of Pump Science and employees' family members are ineligible to participate in this audit.

Code4rena's rules cannot be overridden by the contents of this README. In case of doubt, please check with C4 staff.

# Scope

*See [scope.txt](https://github.com/code-423n4/2025-01-pump-science/blob/main/scope.txt)*

### Files in scope


| File   | Logic Contracts | Interfaces | nSLOC | Purpose | Libraries used |
| ------ | --------------- | ---------- | ----- | -----   | ------------ |
| /programs/pump-science/src/instructions/admin/mod.rs | ****| **** | 4 | ||
| /programs/pump-science/src/state/mod.rs | ****| **** | 4 | ||
| **Totals** | **** | **** | **8** | | |

### Files out of scope

*See [out_of_scope.txt](https://github.com/code-423n4/2025-01-pump-science/blob/main/out_of_scope.txt)*

| File         |
| ------------ |
| ./clients/rust/src/generated/accounts/bonding_curve.rs |
| ./clients/rust/src/generated/accounts/global.rs |
| ./clients/rust/src/generated/accounts/mod.rs |
| ./clients/rust/src/generated/accounts/whitelist.rs |
| ./clients/rust/src/generated/errors/mod.rs |
| ./clients/rust/src/generated/errors/pump_science.rs |
| ./clients/rust/src/generated/instructions/add_wl.rs |
| ./clients/rust/src/generated/instructions/create_bonding_curve.rs |
| ./clients/rust/src/generated/instructions/create_pool.rs |
| ./clients/rust/src/generated/instructions/initialize.rs |
| ./clients/rust/src/generated/instructions/lock_pool.rs |
| ./clients/rust/src/generated/instructions/mod.rs |
| ./clients/rust/src/generated/instructions/remove_wl.rs |
| ./clients/rust/src/generated/instructions/set_params.rs |
| ./clients/rust/src/generated/instructions/swap.rs |
| ./clients/rust/src/generated/mod.rs |
| ./clients/rust/src/generated/programs.rs |
| ./clients/rust/src/generated/types/global_authority_input.rs |
| ./clients/rust/src/generated/types/global_settings_input.rs |
| ./clients/rust/src/generated/types/mod.rs |
| ./clients/rust/src/lib.rs |
| Totals: 21 |

