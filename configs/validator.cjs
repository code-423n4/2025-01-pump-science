const path = require("path");

const programBinDir = path.join(__dirname, "..", ".programsBin");

function getProgram(programBinary) {
  return path.join(programBinDir, programBinary);
}

module.exports = {
  validator: {
    killRunningValidators: true,
    commitment: "processed",
    programs: [
      {
        label: "Pump Science Program",
        programId: "EtZR9gh25YUM6LkL2o2yYV1KzyuDdftHvYk3wsb2Ypkj",
        deployPath: getProgram("pump_science.so"),
      },

      // Below are external programs that should be included in the local validator.
      // You may configure which ones to fetch from the cluster when building
      // programs within the `configs/program-scripts/dump.sh` script.
      {
        label: "Token Metadata",
        programId: "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
        deployPath: getProgram("mpl_token_metadata.so"),
      },
      // {
      //   label: "SPL Noop",
      //   programId: "noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV",
      //   deployPath: getProgram("spl_noop.so"),
      // },
      // {
      //   label: "Spl ATA Program",
      //   programId: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
      //   deployPath: getProgram("spl_ata.so"),
      // },
      // {
      //   label: "SPL Token Program",
      //   programId: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
      //   deployPath: getProgram("spl_token.so"),
      // },
      {
        label: "Mpl System Extras",
        programId: "SysExL2WDyJi9aRZrXorrjHJut3JwHQ7R9bTyctbNNG",
        deployPath: getProgram("mpl_system_extras.so"),
      },
      {
        label: "Dynamic Amm",
        programId: "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB",
        deployPath: getProgram("dynamic_amm.so"),
      },
      {
        label: "Vault",
        programId: "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi",
        deployPath: getProgram("vault.so"),
      },
    ],
    accounts: [{
      label: "Meteora Config",
      accountId: "FiENCCbPi3rFh5pW2AJ59HC53yM32eLaCjMKxRqanKFJ",
      cluster: 'https://api.mainnet-beta.solana.com'
    }, {
      label: "Native Mint",
      accountId: "So11111111111111111111111111111111111111112",
      cluster: 'https://api.mainnet-beta.solana.com'
    }],
    jsonRpcUrl: "localhost",
    websocketUrl: "",
    ledgerDir: "./test-ledger",
    resetLedger: true,
    verifyFees: false,
    detached: true,
  },
  relay: {
    enabled: true,
    killlRunningRelay: true,
  },
  storage: {
    enabled: true,
    storageId: "mock-storage",
    clearOnStart: true,
  },
};
