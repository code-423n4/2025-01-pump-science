Migration test

1. Airdrop some SOL
`solana airdrop 10000 Hce3sP3t82MZFSt42ZmMQMF34sghycvjiQXsSEp6afui`

2. Create Global Config
`pnpm script global -e localhost -r "http://localhost:8899" -k ./pump_test.json`

3. Create Bonding Curve
`pnpm script createCurve -e localhost -r "http://localhost:8899" -k ./pump_test.json`
-> See mint address created

4. Buy all Bonding Curve
`pnpm script swap -e localhost -r "http://localhost:8899" -k ./pump_test.json -m G6xK1rKGA9x4mKc6DVyYtV457ZGJoWrZZjnJ472462J3`
-> -m is mint address from bonding curve creation

5.  Migrate
`pnpm script migrate -e localhost -r "http://localhost:8899" -k ./pump_test.json -m G6xK1rKGA9x4mKc6DVyYtV457ZGJoWrZZjnJ472462J3`
