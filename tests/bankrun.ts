// import { Amman } from "@metaplex-foundation/amman-client";
import {
  keypairIdentity,
  Keypair,
  TransactionBuilder,
  Umi,
  PublicKey,
  publicKey
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  SPL_SYSTEM_PROGRAM_ID, MPL_SYSTEM_EXTRAS_PROGRAM_ID
} from "@metaplex-foundation/mpl-toolbox";
import {
  Connection,
  Keypair as Web3JsKeypair,
  LAMPORTS_PER_SOL,
  PublicKey as Web3JsPublicKey,
  VersionedTransaction,
} from "@solana/web3.js";
import {
  calculateFee,
  CurveSDK,
  PUMP_SCIENCE_PROGRAM_ID,
  PumpScienceSDK,
} from "../clients/js/src";
import {
  fromWeb3JsKeypair,
  toWeb3JsPublicKey,
  toWeb3JsTransaction,
} from "@metaplex-foundation/umi-web3js-adapters";
import { BankrunProvider } from "anchor-bankrun";
import assert from "assert";
import {
  INIT_DEFAULTS,
  SIMPLE_DEFAULT_BONDING_CURVE_PRESET
} from "../clients/js/src/constants";
import { AMM } from "../clients/js/src/amm";
import {
  BanksClient,
  Clock,
  ProgramTestContext,
  startAnchor,
} from "solana-bankrun";
import { web3JsRpc } from "@metaplex-foundation/umi-rpc-web3js";
import { AccountLayout } from "@solana/spl-token";
import { readFileSync } from "fs";
import path from "path";
import { BN } from "bn.js";

const USE_BANKRUN = true;
const INITIAL_SOL = 500 * LAMPORTS_PER_SOL;

// const amman = Amman.instance({
//   ammanClientOpts: { autoUnref: false, ack: true },
//   knownLabels: {
//     [PUMP_SCIENCE_PROGRAM_ID.toString()]: "PumpScienceProgram",
//   },
// });
const MPL_TOKEN_METADATA_PROGRAM_ID = publicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
// --- KEYPAIRS
const web3Keypair = Web3JsKeypair.fromSecretKey(Uint8Array.from(require("../pump_test.json")))
const masterKp = fromWeb3JsKeypair(
  web3Keypair
);
const simpleMintKp = fromWeb3JsKeypair(Web3JsKeypair.generate());
const creator = fromWeb3JsKeypair(Web3JsKeypair.generate());
const trader = fromWeb3JsKeypair(Web3JsKeypair.generate());
const withdrawAuthority = fromWeb3JsKeypair(Web3JsKeypair.generate());

// amman.addr.addLabel("withdrawAuthority", withdrawAuthority.publicKey);
// amman.addr.addLabel("simpleMint", simpleMintKp.publicKey);
// amman.addr.addLabel("creator", creator.publicKey);
// amman.addr.addLabel("trader", trader.publicKey);

// --- PROVIDERS
let bankrunContext: ProgramTestContext;
let bankrunClient: BanksClient;
let bankrunProvider: BankrunProvider;
let connection: Connection;
let rpcUrl = "http://localhost:8899";

let umi: Umi;

const programBinDir = path.join(__dirname, "..", ".programsBin");

function getProgram(programBinary) {
  return path.join(programBinDir, programBinary);
}
const loadProviders = async () => {
  process.env.ANCHOR_WALLET = "../pump_test.json";
  bankrunContext = await startAnchor(
    "./",
    [],
    [
      {
        address: toWeb3JsPublicKey(masterKp.publicKey),
        info: {
          lamports: INITIAL_SOL,
          executable: false,
          data: Buffer.from([]),
          owner: toWeb3JsPublicKey(SPL_SYSTEM_PROGRAM_ID),
        },
      },
      {
        address: toWeb3JsPublicKey(creator.publicKey),
        info: {
          lamports: INITIAL_SOL,
          executable: false,
          data: Buffer.from([]),
          owner: toWeb3JsPublicKey(SPL_SYSTEM_PROGRAM_ID),
        },
      },
      {
        address: toWeb3JsPublicKey(trader.publicKey),
        info: {
          lamports: INITIAL_SOL,
          executable: false,
          data: Buffer.from([]),
          owner: toWeb3JsPublicKey(SPL_SYSTEM_PROGRAM_ID),
        },
      },
      {
        address: toWeb3JsPublicKey(withdrawAuthority.publicKey),
        info: {
          lamports: INITIAL_SOL,
          executable: false,
          data: Buffer.from([]),
          owner: toWeb3JsPublicKey(SPL_SYSTEM_PROGRAM_ID),
        },
      },
      {
        address: toWeb3JsPublicKey(MPL_TOKEN_METADATA_PROGRAM_ID),
        info: await loadBin(getProgram("mpl_token_metadata.so")),
      },
      {
        address: toWeb3JsPublicKey(MPL_SYSTEM_EXTRAS_PROGRAM_ID),
        info: await loadBin(getProgram("mpl_system_extras.so")),
      },
    ]
  );
  console.log("bankrunCtx: ", bankrunContext);
  bankrunClient = bankrunContext.banksClient;
  bankrunProvider = new BankrunProvider(bankrunContext);

  console.log("anchor connection: ", bankrunProvider.connection.rpcEndpoint);

  //@ts-ignore
  bankrunProvider.connection.rpcEndpoint = rpcUrl;
  const conn = bankrunProvider.connection;

  umi = createUmi(rpcUrl).use(web3JsRpc(conn));
  connection = conn;
  console.log("using bankrun payer");
};

export const loadBin = async (binPath: string) => {
  const programBytes = readFileSync(binPath);
  const executableAccount = {
    lamports: INITIAL_SOL,
    executable: true,
    owner: new Web3JsPublicKey("BPFLoader2111111111111111111111111111111111"),
    data: programBytes,
  };
  return executableAccount;
};

// pdas and util accs

// const labelKeypairs = async (umi) => {
// amman.addr.addLabel("master", masterKp.publicKey);
// amman.addr.addLabel("simpleMint", simpleMintKp.publicKey);
// amman.addr.addLabel("creator", creator.publicKey);
// amman.addr.addLabel("trader", trader.publicKey);
// amman.addr.addLabel("withdrawAuthority", withdrawAuthority.publicKey);

// const curveSdk = new PumpScienceSDK(
//   // master signer
//   umi.use(keypairIdentity(masterKp))
// ).getCurveSDK(simpleMintKp.publicKey);

// amman.addr.addLabel("global", curveSdk.PumpScience.globalPda[0]);
// amman.addr.addLabel("eventAuthority", curveSdk.PumpScience.evtAuthPda[0]);
// amman.addr.addLabel("simpleMintBondingCurve", curveSdk.bondingCurvePda[0]);
// amman.addr.addLabel(
//   "simpleMintBondingCurveTknAcc",
//   curveSdk.bondingCurveTokenAccount[0]
// );
// amman.addr.addLabel("metadata", curveSdk.mintMetaPda[0]);
// };

import { transactionBuilder } from "@metaplex-foundation/umi";
import { setComputeUnitLimit } from "@metaplex-foundation/mpl-toolbox";

async function processTransaction(umi, txBuilder: TransactionBuilder) {
  let txWithBudget = await transactionBuilder().add(
    setComputeUnitLimit(umi, { units: 600_000 })
  );

  const fullBuilder = txBuilder.prepend(txWithBudget);
  if (USE_BANKRUN) {
    let tx: VersionedTransaction;
    try {
      const bhash = await bankrunClient.getLatestBlockhash();
      tx = toWeb3JsTransaction(
        await fullBuilder.setBlockhash(bhash?.[0] || "").build(umi)
      );
    } catch (error) {
      console.log("error: ", error);
      throw error;
    }
    return await bankrunClient.processTransaction(tx);
  } else {
    return await fullBuilder.sendAndConfirm(umi);
  }
}

const getBalance = async (umi: Umi, pubkey: PublicKey) => {
  // cannot use umi helpers in bankrun
  if (USE_BANKRUN) {
    const balance = await bankrunClient.getBalance(toWeb3JsPublicKey(pubkey));
    return balance;
  } else {
    const umiBalance = await umi.rpc.getBalance(pubkey);
    return umiBalance.basisPoints;
  }
};

const getTknAmount = async (umi: Umi, pubkey: PublicKey) => {
  // cannot use umi helpers and some rpc methods in bankrun
  if (USE_BANKRUN) {
    const accInfo = await bankrunClient.getAccount(toWeb3JsPublicKey(pubkey));
    const info = AccountLayout.decode(accInfo?.data || Buffer.from([]));
    return info.amount;
  } else {
    const umiBalance = await connection.getAccountInfo(
      toWeb3JsPublicKey(pubkey)
    );
    const info = AccountLayout.decode(umiBalance?.data || Buffer.from([]));
    return info.amount;
  }
};

describe("pump-science", () => {
  before(async () => {
    await loadProviders();
    // await labelKeypairs(umi);
  });

  it("is initialized", async () => {
    const adminSdk = new PumpScienceSDK(
      // admin signer
      umi.use(keypairIdentity(masterKp))
    ).getAdminSDK();

    const txBuilder = adminSdk.initialize({ ...INIT_DEFAULTS, whitelistEnabled: true });
    await processTransaction(umi, txBuilder);

    const global = await adminSdk.PumpScience.fetchGlobalData();
    console.log("global", global);
  });

  it("is update wl: add", async () => {
    const wlSdk = new PumpScienceSDK(
      // admin signer
      umi.use(keypairIdentity(masterKp))
    ).getWlSDK(creator.publicKey);

    const txBuilder = wlSdk.addWl();

    await processTransaction(umi, txBuilder);

    const wl = await wlSdk.fetchWlData();
    console.log("whitelist data ===>>>", wl);
    console.log("Creator wl", creator.publicKey);

  });

  it("creates simple bonding curve", async () => {
    const curveSdk = new PumpScienceSDK(
      // creator signer
      umi.use(keypairIdentity(creator))
    ).getCurveSDK(simpleMintKp.publicKey);

    console.log("globalPda[0]", curveSdk.PumpScience.globalPda[0]);
    console.log("bondingCurvePda[0]", curveSdk.bondingCurvePda[0]);
    console.log("bondingCurveTknAcc[0]", curveSdk.bondingCurveTokenAccount[0]);
    console.log("metadataPda[0]", curveSdk.mintMetaPda[0]);

    const txBuilder = curveSdk.createBondingCurve(
      SIMPLE_DEFAULT_BONDING_CURVE_PRESET,
      // needs the mint Kp to create the curve
      simpleMintKp,
      true
    );

    await processTransaction(umi, txBuilder);

    const bondingCurveData = await curveSdk.fetchData();
    console.log("bondingCurveData", bondingCurveData);
  });

  it("is update wl: remove", async () => {
    const wlSdk = new PumpScienceSDK(
      // admin signer
      umi.use(keypairIdentity(masterKp))
    ).getWlSDK(creator.publicKey);

    const txBuilder = wlSdk.removeWl();

    await processTransaction(umi, txBuilder);

    // const wl = await wlSdk.fetchWlData();
    // console.log("whitelist data ===>>>", wl);
  });

  it("creator try to create another bonding curve should fail", async () => {
    const mintKp = umi.eddsa.generateKeypair();

    const curveSdk = new PumpScienceSDK(
      // creator signer 
      umi.use(keypairIdentity(creator))
    ).getCurveSDK(mintKp.publicKey);

    const txBuilder = curveSdk.createBondingCurve(
      SIMPLE_DEFAULT_BONDING_CURVE_PRESET,
      // needs the mint Kp to create the curve
      mintKp,
      false
    );

    try {
      await processTransaction(umi, txBuilder)
      assert(false, "Should have failed");
    } catch (e) {
      // Expected error
      console.log("Expected error creating duplicate curve:", e);
    }
  });

  it("swap: buy", async () => {
    const executeBuy = async (curveSdk: CurveSDK, solAmount: bigint, slot: bigint, devBuy: boolean = false) => {
      const bondingCurveData = await curveSdk.fetchData();
      const amm = AMM.fromBondingCurve(bondingCurveData);

      const { solFee, feeBps } = calculateFee(Number(solAmount), Number(slot), Number(bondingCurveData.startSlot));
      console.log("fee", solFee);
      console.log("feeBps", feeBps);
      const appliedSolAmount = devBuy ? solAmount : solAmount - BigInt(solFee);

      // should use actual fee set on global when live
      let buyResult = amm.applyBuy(appliedSolAmount);
      console.log("buySimResult", buyResult);

      // Apply 5% slippage
      let minOutAmount = buyResult.token_amount - BigInt(solFee) - (buyResult.token_amount * 5n) / 100n;

      const txBuilder = curveSdk.swap({
        direction: "buy",
        exactInAmount: solAmount,
        minOutAmount: minOutAmount,
      });

      await processTransaction(umi, txBuilder);

      const bondingCurveDataPost = await curveSdk.fetchData();
      const traderAtaBalancePost = await getTknAmount(
        umi,
        curveSdk.userTokenAccount[0]
      );

      console.log("pre.realTokenReserves", bondingCurveData.realTokenReserves);
      console.log(
        "post.realTokenReserves",
        bondingCurveDataPost.realTokenReserves
      );
      console.log("buyTokenAmount", minOutAmount);
      const tknAmountDiff = BigInt(
        bondingCurveData.realTokenReserves -
        bondingCurveDataPost.realTokenReserves
      );
      console.log("real difference", tknAmountDiff);
      console.log(
        "buyAmount-tknAmountDiff",
        tknAmountDiff - minOutAmount,
        tknAmountDiff > minOutAmount
      );
      assert(tknAmountDiff > minOutAmount);

      console.log("pre.realSolReserves", bondingCurveData.realSolReserves);
      console.log("post.realSolReserves", bondingCurveDataPost.realSolReserves);
      console.log("appliedSolAmount", appliedSolAmount);

      assert(
        bondingCurveDataPost.realSolReserves ==
        bondingCurveData.realSolReserves + appliedSolAmount
      );
      assert(traderAtaBalancePost >= minOutAmount);
    }

    // Execute multiple buys
    const curveSdkCreator = new PumpScienceSDK(
      // creator signer
      umi.use(keypairIdentity(creator))
    ).getCurveSDK(simpleMintKp.publicKey);


    // dev buy
    const solAmount = 10_000_000_000n; // 10 SOL

    let slot = await bankrunClient.getSlot()
    console.log("slot first buy", slot)

    await executeBuy(curveSdkCreator, solAmount, slot, true)

    // first slot buy non-creator
    const curveSdkTrader = new PumpScienceSDK(
      // trader signer
      umi.use(keypairIdentity(trader))
    ).getCurveSDK(simpleMintKp.publicKey);
    await executeBuy(curveSdkTrader, solAmount, slot, false)

    // fast forward to start of linear fee decay phase
    bankrunContext.warpToSlot(151n)
    slot = await bankrunClient.getSlot()
    console.log("slot second buy", slot)
    await executeBuy(curveSdkTrader, 1_000_000_000n, slot)

    // Test normal creator buy
    bankrunContext.warpToSlot(201n)
    slot = await bankrunClient.getSlot()
    console.log("slot third buy", slot)
    await executeBuy(curveSdkTrader, 1_000_000_000n, slot)

    bankrunContext.warpToSlot(251n)
    slot = await bankrunClient.getSlot()
    console.log("slot fourth buy", slot)

    await executeBuy(curveSdkTrader, 1_000_000_000n, slot)

    bankrunContext.warpToSlot(301n)
    slot = await bankrunClient.getSlot()
    console.log("slot fifth buy", slot)
    await executeBuy(curveSdkTrader, 1_000_000_000n, slot)
  });

  it("swap: sell", async () => {
    const curveSdk = new PumpScienceSDK(
      // trader signer
      umi.use(keypairIdentity(creator))
    ).getCurveSDK(simpleMintKp.publicKey);

    const bondingCurveData = await curveSdk.fetchData();
    console.log("bondingCurveData", bondingCurveData);
    const traderAtaBalancePre = await getTknAmount(
      umi,
      curveSdk.userTokenAccount[0]
    );

    const amm = AMM.fromBondingCurve(bondingCurveData);
    let sellTokenAmount = 100_000_000_000n; // 100,000 Tokens -> 0.01% total supply

    let sellResult = amm.applySell(sellTokenAmount);
    console.log("sellSimResult", sellResult);

    const currentSlot = await bankrunClient.getSlot()

    const { solFee, feeBps } = calculateFee(Number(sellResult.sol_amount), Number(currentSlot), Number(bondingCurveData.startSlot));
    console.log("fee", solFee);
    console.log("feeBps", feeBps);

    const appliedSolAmount = sellResult.sol_amount - BigInt(solFee)

    // Apply 5% slippage
    let minOutAmount = appliedSolAmount - (appliedSolAmount * 5n) / 100n

    console.log("minOutAmount", minOutAmount);

    const txBuilder = curveSdk.swap({
      direction: "sell",
      exactInAmount: sellTokenAmount,
      minOutAmount: minOutAmount,
    });

    await processTransaction(umi, txBuilder);

    // Post-transaction checks
    const bondingCurveDataPost = await curveSdk.fetchData();
    const traderAtaBalancePost = await getTknAmount(
      umi,
      curveSdk.userTokenAccount[0]
    );
    assert(
      bondingCurveDataPost.realTokenReserves ==
      bondingCurveData.realTokenReserves + sellTokenAmount
    );
    assert(traderAtaBalancePost == traderAtaBalancePre - sellTokenAmount);
  });

  it("set_params: migrateFeeAmount", async () => {
    const adminSdk = new PumpScienceSDK(
      // admin signer
      umi.use(keypairIdentity(masterKp))
    ).getAdminSDK();

    const globalData = await adminSdk.PumpScience.fetchGlobalData();

    const txBuilder = adminSdk.setParams({
      migrateFeeAmount: 500,
      // Required to always include even if they are not being changed
      initialRealTokenReserves: globalData.initialRealTokenReserves,
      initialVirtualTokenReserves: globalData.initialVirtualTokenReserves,
      initialVirtualSolReserves: globalData.initialVirtualSolReserves,
      tokenTotalSupply: globalData.tokenTotalSupply,
      mintDecimals: globalData.mintDecimals,
      feeReceiver: globalData.feeReceiver,
      whitelistEnabled: globalData.whitelistEnabled,
      meteoraConfig: globalData.meteoraConfig,
      migrationTokenAllocation: globalData.migrationTokenAllocation,
    });

    await processTransaction(umi, txBuilder);
    const global = await adminSdk.PumpScience.fetchGlobalData();
    console.log("Global Data", global);
  });
});
