# Migration

At migration we create a meteora pool and consecutively lock the LP tokens. The authority/payer/signer for the meteora CPI calls is the sol_escrow PDA account.

The Bonding Curve itself doesn't work since it's a data account and cannot act as the payer for rent costs of e.g. creating the LP mint and metadata within the CPI call to meteora

**Setup Create Pool Ix from Program**
In this instruction we prepare our completed bonding curve and do a meteora CPI call to create a pool

1. Prepare Tokens and SOL
    - Transfer Tokens in `bonding_curve` PDA owned ata to new ata belonging to the `sol_escrow`account.
    - Transfer native SOL from `sol_escrow` to wrapped SOL ata and call `sync` instruction to convert SOL to wrapped SOL
2. CPI call to meteora with `sol_escrow` as signer
3. Transfer Migration fee to `fee_receiver`

**Setup Lock Ix from Program**
In this instruction we create the LP escrow account, the respective LP token ata for the escrow account, and finally lock the LP tokens in the escrow account with the `fee_payer` role as authority to claim fees

Again `sol_escrow` is the signer of the cpi call because sol_escrow is the current owner of the LP tokens after this account has created the pool in the previous step.


**Transaction Requirements**

Due to the complexity of the migration process and the number of accounts involved, the migration instructions require special transaction handling:

1. **Version 0 Transactions**
   - Migration instructions must use Transaction Version 0 (V0) format
   - V0 transactions support Address Lookup Tables which are needed to handle the large number of accounts

2. **Address Lookup Tables (LUTs)**
   - LUTs must be created and deployed before migration
   - Required due to account address limit in regular transactions
   - Stores frequently referenced account addresses to reduce transaction size
   - Critical accounts stored in LUT include:
     - Program IDs (Meteora, Token, Associated Token)
     - Pool configuration accounts
     - Token mint addresses
     - PDAs and authority accounts

The CLI implementation handles creating and managing the necessary LUTs before executing the migration instructions.
