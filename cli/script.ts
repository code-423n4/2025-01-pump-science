import { BN, Program, web3 } from '@coral-xyz/anchor';
import * as anchor from '@coral-xyz/anchor';
import fs from 'fs';
import NodeWallet from '@coral-xyz/anchor/dist/cjs/nodewallet';
import { INIT_DEFAULTS, SIMPLE_DEFAULT_BONDING_CURVE_PRESET, FEE_RECEIVER, METEORA_CONFIG, PumpScienceSDK, PUMP_SCIENCE_PROGRAM_ID } from '../clients/js/src';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { web3JsRpc } from '@metaplex-foundation/umi-rpc-web3js';
import { fromWeb3JsKeypair, toWeb3JsPublicKey } from '@metaplex-foundation/umi-web3js-adapters';
import { keypairIdentity, publicKey, transactionBuilder, TransactionBuilder, Umi } from '@metaplex-foundation/umi';
import { setComputeUnitLimit } from '@metaplex-foundation/mpl-toolbox';
import { ComputeBudgetProgram, Transaction, PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram, TransactionInstruction, TransactionMessage, VersionedTransaction, LAMPORTS_PER_SOL, SYSVAR_CLOCK_PUBKEY, AddressLookupTableProgram, Keypair as Web3JsKeypair } from '@solana/web3.js';
import VaultImpl, { getVaultPdas } from '@mercurial-finance/vault-sdk';
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, NATIVE_MINT, getAssociatedTokenAddressSync } from '@solana/spl-token';
import AmmImpl, { PROGRAM_ID } from '@mercurial-finance/dynamic-amm-sdk';
import { IDL, PumpScience } from '../target/types/pump_science';
import { vault, derivePoolAddressWithConfig, createProgram, getOrCreateATAInstruction, deriveMintMetadata, wrapSOLInstruction, deriveLockEscrowPda } from './util'
import { SEEDS } from '@mercurial-finance/dynamic-amm-sdk/dist/cjs/src/amm/constants';
import { METAPLEX_PROGRAM } from '@mercurial-finance/dynamic-amm-sdk/dist/cjs/src/amm/constants';
import { bs58 } from '@coral-xyz/anchor/dist/cjs/utils/bytes';

let solConnection: web3.Connection = null;
let program: anchor.Program<PumpScience> = null;
let provider: anchor.Provider = null;
let payer: NodeWallet = null;
let umi: Umi;

const simpleMintKp = Web3JsKeypair.generate();

// Address of the deployed program.
let programId = toWeb3JsPublicKey(PUMP_SCIENCE_PROGRAM_ID);
/**
 * Set cluster, provider, program
 * If rpc != null use rpc, otherwise use cluster param
 * @param cluster - cluster ex. mainnet-beta, devnet ...
 * @param keypair - wallet keypair
 * @param rpc - rpc
 */
export const setClusterConfig = async (
    cluster: web3.Cluster,
    keypair: string,
    rpc?: string
) => {
    if (!rpc) {
        solConnection = new web3.Connection(web3.clusterApiUrl(cluster));
    } else {
        console.log("Using RPC:", rpc);
        solConnection = new web3.Connection(rpc);
    }

    const walletKeypair = web3.Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(fs.readFileSync(keypair, 'utf-8'))),
        { skipValidation: true });

    const wallet = new NodeWallet(walletKeypair);

    // Configure the client to use the local cluster.
    anchor.setProvider(new anchor.AnchorProvider(
        solConnection,
        wallet,
        { skipPreflight: true, commitment: 'processed' }));
    payer = wallet;

    provider = anchor.getProvider();
    const rpcUrl = rpc ? rpc : web3.clusterApiUrl(cluster);
    umi = createUmi(rpcUrl).use(web3JsRpc(provider.connection));

    // Generate the program client from IDL.
    program = new anchor.Program(IDL as PumpScience, programId);
}

export const global = async () => {

    const global = PublicKey.findProgramAddressSync([Buffer.from("global")], programId)[0];
    const eventAuthority = PublicKey.findProgramAddressSync([Buffer.from("__event_authority")], programId)[0];
    console.log("global", global.toBase58());

    const initDefaults = {
        initialVirtualTokenReserves: new BN(INIT_DEFAULTS.initialVirtualTokenReserves),
        initialVirtualSolReserves: new BN(INIT_DEFAULTS.initialVirtualSolReserves),
        initialRealTokenReserves: new BN(INIT_DEFAULTS.initialRealTokenReserves),
        tokenTotalSupply: new BN(INIT_DEFAULTS.tokenTotalSupply),
        mintDecimals: INIT_DEFAULTS.mintDecimals,
        migrateFeeAmount: new BN(INIT_DEFAULTS.migrateFeeAmount),
        migrationTokenAllocation: new BN(INIT_DEFAULTS.migrationTokenAllocation),
        whitelistEnabled: INIT_DEFAULTS.whitelistEnabled,
        feeReceiver: toWeb3JsPublicKey(INIT_DEFAULTS.feeReceiver),
        meteoraConfig: toWeb3JsPublicKey(INIT_DEFAULTS.meteoraConfig)
    };

    const tx = await program.methods.initialize(initDefaults).accounts({
        global,
        eventAuthority,
        systemProgram: SystemProgram.programId,
        program: programId
    }).transaction();

    const latestBlockHash = await provider.connection.getLatestBlockhash(
        provider.connection.commitment,
    );
    const creatTx = new web3.Transaction({
        feePayer: payer.publicKey,
        ...latestBlockHash,
    }).add(tx)

    creatTx.sign(payer.payer);

    const preInxSim = await solConnection.simulateTransaction(creatTx)

    const txHash = await provider.sendAndConfirm(creatTx, [], {
        commitment: "finalized",
    });

    return txHash;
}

export const migrate = async (mint: string) => {
    const { ammProgram, vaultProgram } = createProgram(provider.connection, null);
    const eventAuthority = PublicKey.findProgramAddressSync([Buffer.from("__event_authority")], new PublicKey(PROGRAM_ID))[0];

    const global = PublicKey.findProgramAddressSync([Buffer.from("global")], programId)[0];

    const tokenAMint = NATIVE_MINT;

    // Needs to be dynamic
    const tokenBMint = new PublicKey(mint);

    // Needs to as defined in smart contract
    const config = toWeb3JsPublicKey(METEORA_CONFIG);
    const feeReceiver = toWeb3JsPublicKey(FEE_RECEIVER);

    const bondingCurve = PublicKey.findProgramAddressSync([Buffer.from("bonding-curve"), tokenBMint.toBuffer()], programId)[0];

    const poolPubkey = derivePoolAddressWithConfig(tokenAMint, tokenBMint, config, ammProgram.programId);

    const [
        { vaultPda: aVault, tokenVaultPda: aTokenVault, lpMintPda: aLpMintPda },
        { vaultPda: bVault, tokenVaultPda: bTokenVault, lpMintPda: bLpMintPda },
    ] = [getVaultPdas(tokenAMint, vaultProgram.programId), getVaultPdas(tokenBMint, vaultProgram.programId)];

    let aVaultLpMint = aLpMintPda;
    let bVaultLpMint = bLpMintPda;
    let preInstructions: Array<TransactionInstruction> = [];

    // Vault creation Ixs
    const [aVaultAccount, bVaultAccount] = await Promise.all([
        vaultProgram.account.vault.fetchNullable(aVault),
        vaultProgram.account.vault.fetchNullable(bVault),
    ]);

    if (!aVaultAccount) {
        const createVaultAIx = await VaultImpl.createPermissionlessVaultInstruction(provider.connection, payer.publicKey, tokenAMint);
        createVaultAIx && preInstructions.push(createVaultAIx);

    } else {
        aVaultLpMint = aVaultAccount.lpMint; // Old vault doesn't have lp mint pda
    }
    if (!bVaultAccount) {
        const createVaultBIx = await VaultImpl.createPermissionlessVaultInstruction(provider.connection, payer.publicKey, tokenBMint);
        createVaultBIx && preInstructions.push(createVaultBIx);

    } else {
        bVaultLpMint = bVaultAccount.lpMint; // Old vault doesn't have lp mint pda
    }

    const [lpMint] = PublicKey.findProgramAddressSync(
        [Buffer.from(SEEDS.LP_MINT), poolPubkey.toBuffer()],
        ammProgram.programId,
    );
    const [[aVaultLp], [bVaultLp]] = [
        PublicKey.findProgramAddressSync([aVault.toBuffer(), poolPubkey.toBuffer()], ammProgram.programId),
        PublicKey.findProgramAddressSync([bVault.toBuffer(), poolPubkey.toBuffer()], ammProgram.programId),
    ];

    // Payer for the Meteora Pool creation will be the sol_escrow PDA of the program
    const [bondingCurveSolEscrow] = PublicKey.findProgramAddressSync([Buffer.from("sol-escrow"), tokenBMint.toBuffer()], programId);

    const [[payerTokenB, payerTokenBIx], [payerTokenA, payerTokenAIx]] = await Promise.all([
        getOrCreateATAInstruction(tokenBMint, bondingCurveSolEscrow, provider.connection, payer.publicKey),
        getOrCreateATAInstruction(tokenAMint, bondingCurveSolEscrow, provider.connection, payer.publicKey),
    ]);


    // Create Native Mint SOL ATA for sol escrow
    payerTokenAIx && preInstructions.push(payerTokenAIx);
    payerTokenBIx && preInstructions.push(payerTokenBIx);

    const [feeReceiverTokenAccount, feeReceiverTokenAccountIx] = await getOrCreateATAInstruction(tokenBMint, feeReceiver, provider.connection, payer.publicKey);
    feeReceiverTokenAccountIx && preInstructions.push(feeReceiverTokenAccountIx);


    const bondingCurveTokenB = getAssociatedTokenAddressSync(tokenBMint, bondingCurve, true);

    const [[protocolTokenAFee], [protocolTokenBFee]] = [
        PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.FEE), tokenAMint.toBuffer(), poolPubkey.toBuffer()],
            ammProgram.programId,
        ),
        PublicKey.findProgramAddressSync(
            [Buffer.from(SEEDS.FEE), tokenBMint.toBuffer(), poolPubkey.toBuffer()],
            ammProgram.programId,
        ),
    ];

    // LP ata of bonding curve
    const payerPoolLp = getAssociatedTokenAddressSync(lpMint, bondingCurveSolEscrow, true);

    const setComputeUnitLimitIx = ComputeBudgetProgram.setComputeUnitLimit({
        units: 20_000_000,
    });
    let latestBlockHash = await provider.connection.getLatestBlockhash(
        provider.connection.commitment,
    );

    if (preInstructions.length) {
        const preInstructionTx = new Transaction({
            feePayer: payer.publicKey,
            ...latestBlockHash,
        }).add(...preInstructions);

        preInstructionTx.sign(payer.payer);
        const preInxSim = await solConnection.simulateTransaction(preInstructionTx)

        const txHash = await provider.sendAndConfirm(preInstructionTx, [], {
            commitment: "finalized",
        });
    }

    const [mintMetadata, _mintMetadataBump] = deriveMintMetadata(lpMint);
    const [tokenBMetadata, _tokenBMetadataBump] = deriveMintMetadata(lpMint);

    // Escrow for claim authority fee-receiver
    const [lockEscrowPK] = deriveLockEscrowPda(poolPubkey, feeReceiver, ammProgram.programId);
    const [escrowAta, createEscrowAtaIx] = await getOrCreateATAInstruction(lpMint, lockEscrowPK, solConnection, payer.publicKey);

    const txLockPool = await program.methods
        .lockPool()
        .accounts({
            global,
            bondingCurve,
            bondingCurveSolEscrow,
            pool: poolPubkey,
            lpMint,
            aVaultLp,
            bVaultLp,
            tokenBMint,
            aVault,
            bVault,
            aVaultLpMint,
            bVaultLpMint,
            payerPoolLp,
            payer: payer.publicKey,
            feeReceiver,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            lockEscrow: lockEscrowPK,
            escrowVault: escrowAta,
            meteoraProgram: PROGRAM_ID,
            eventAuthority
        })
        .transaction();
    const txCreatePool = await program.methods
        .createPool()
        .accounts({
            global,
            bondingCurve,
            feeReceiver,
            feeReceiverTokenAccount,
            pool: poolPubkey,
            config,
            lpMint,
            aVaultLp,
            bVaultLp,
            tokenAMint,
            tokenBMint,
            aVault,
            bVault,
            aTokenVault,
            bTokenVault,
            aVaultLpMint,
            bVaultLpMint,
            bondingCurveTokenAccount: bondingCurveTokenB,
            bondingCurveSolEscrow,
            payerTokenA,
            payerTokenB,
            payerPoolLp,
            protocolTokenAFee,
            protocolTokenBFee,
            payer: payer.publicKey,
            mintMetadata,
            rent: SYSVAR_RENT_PUBKEY,
            metadataProgram: METAPLEX_PROGRAM,
            vaultProgram: vaultProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            meteoraProgram: PROGRAM_ID,
        })
        .transaction();

    /// create meteora pool ///
    const creatTx = new web3.Transaction({
        feePayer: payer.publicKey,
        ...latestBlockHash,
    }).add(setComputeUnitLimitIx).add(txCreatePool)

    const slot = await provider.connection.getSlot()

    const [lookupTableInst, lookupTableAddress] =
        AddressLookupTableProgram.createLookupTable({
            authority: payer.publicKey,
            payer: payer.publicKey,
            recentSlot: slot - 200,
        });

    const addresses = [
        global,
        bondingCurve,
        feeReceiver,
        feeReceiverTokenAccount,
        poolPubkey,
        config,
        lpMint,
        tokenAMint,
        tokenBMint,
        aVault,
        bVault,
        aTokenVault,
        bTokenVault,
        aVaultLp,
        bVaultLp,
        aVaultLpMint,
        bVaultLpMint,
        payerTokenA,
        payerTokenB,
        bondingCurveSolEscrow,
        payerPoolLp,
        protocolTokenAFee,
        protocolTokenBFee,
        payer.publicKey,
        mintMetadata,
        SYSVAR_RENT_PUBKEY,
        METAPLEX_PROGRAM,
        vaultProgram.programId,
        TOKEN_PROGRAM_ID,
        ASSOCIATED_TOKEN_PROGRAM_ID,
        SystemProgram.programId,
        new PublicKey(PROGRAM_ID),
    ]

    const addAddressesInstruction1 = AddressLookupTableProgram.extendLookupTable({
        payer: payer.publicKey,
        authority: payer.publicKey,
        lookupTable: lookupTableAddress,
        addresses: addresses.slice(0, 30)
    });

    latestBlockHash = await provider.connection.getLatestBlockhash(
        provider.connection.commitment,
    );

    const lutMsg1 = new TransactionMessage({
        payerKey: payer.publicKey,
        recentBlockhash: latestBlockHash.blockhash,
        instructions: [lookupTableInst, addAddressesInstruction1]
    }).compileToV0Message();

    const lutVTx1 = new VersionedTransaction(lutMsg1);
    lutVTx1.sign([payer.payer])

    const lutId1 = await provider.connection.sendTransaction(lutVTx1)
    const lutConfirm1 = await provider.connection.confirmTransaction(lutId1, 'finalized')
    await sleep(2000);
    const lookupTableAccount = await provider.connection.getAddressLookupTable(lookupTableAddress, { commitment: 'finalized' })

    const createTxMsg = new TransactionMessage({
        payerKey: payer.publicKey,
        recentBlockhash: latestBlockHash.blockhash,
        instructions: creatTx.instructions
    }).compileToV0Message([lookupTableAccount.value]);

    const createVTx = new VersionedTransaction(createTxMsg);
    createVTx.sign([payer.payer])

    const sim = await provider.connection.simulateTransaction(createVTx, { sigVerify: true })

    console.log('sim', sim)
    const id = await provider.connection.sendTransaction(createVTx, { skipPreflight: false })
    console.log('id', id)
    const confirm = await provider.connection.confirmTransaction(id)
    console.log('confirm', confirm)

    //// lock pool /////
    const lockPoolTxMsg = new TransactionMessage({
        payerKey: payer.publicKey,
        recentBlockhash: latestBlockHash.blockhash,
        instructions: txLockPool.instructions
        // }).compileToV0Message();
    }).compileToV0Message([lookupTableAccount.value]);

    const lockPoolVTx = new VersionedTransaction(lockPoolTxMsg);
    lockPoolVTx.sign([payer.payer])

    const lockPoolSim = await provider.connection.simulateTransaction(lockPoolVTx, { sigVerify: true })
    console.log('lockPoolSim', lockPoolSim)
    const lockPoolId = await provider.connection.sendTransaction(lockPoolVTx, { skipPreflight: true })
    console.log('lockPoolId', lockPoolId)
    const lockPoolConfirm = await provider.connection.confirmTransaction(lockPoolId)
    console.log('lockPoolConfirm', lockPoolConfirm)

    return lockPoolId;
}

export const createBondingCurve = async () => {

    const web3Keypair = Web3JsKeypair.fromSecretKey(Uint8Array.from(require("../pump_test.json")))
    const masterKp = fromWeb3JsKeypair(
        web3Keypair
    );
    const curveSdk = new PumpScienceSDK(
        // creator signer
        umi.use(keypairIdentity(masterKp))
    ).getCurveSDK(publicKey(simpleMintKp.publicKey.toBase58()));

    const txBuilder = curveSdk.createBondingCurve(
        SIMPLE_DEFAULT_BONDING_CURVE_PRESET,
        fromWeb3JsKeypair(simpleMintKp),
        false
    );

    const sig = await processTransaction(umi, txBuilder);
    console.log("successfully created! ", bs58.encode(sig.signature));

    const bondingCurveData = await curveSdk.fetchData();
    console.log("bondingCurveData", bondingCurveData);
}

export const addWl = async () => {
    const web3Keypair = Web3JsKeypair.fromSecretKey(Uint8Array.from(require("../pump_test.json")))
    const masterKp = fromWeb3JsKeypair(
        web3Keypair
    );
    const creator = masterKp;
    const wlSdk = new PumpScienceSDK(
        // creator signer
        umi.use(keypairIdentity(masterKp))
    ).getWlSDK(publicKey(creator.publicKey));

    const txBuilder = wlSdk.addWl();

    await processTransaction(umi, txBuilder);
    console.log("Added whitelist!");
}

export const swap = async (mint: string) => {
    const web3Keypair = Web3JsKeypair.fromSecretKey(Uint8Array.from(require("../pump_test.json")))
    const masterKp = fromWeb3JsKeypair(
        web3Keypair
    );

    const curveSdk = new PumpScienceSDK(
        // creator signer
        umi.use(keypairIdentity(masterKp))
    ).getCurveSDK(publicKey(mint));

    // Try buy full amount
    const txBuilder = curveSdk.swap({ direction: "buy", exactInAmount: 100000000000, minOutAmount: 1000 });

    console.log("txBuilder", txBuilder);

    const sig = await processTransaction(umi, txBuilder);
    console.log("Swapped! ", bs58.encode(sig.signature));
}

async function processTransaction(umi, txBuilder: TransactionBuilder) {
    let txWithBudget = transactionBuilder().add(
        setComputeUnitLimit(umi, { units: 6_000_000 })
    );

    const fullBuilder = txBuilder.prepend(txWithBudget);

    return await fullBuilder.sendAndConfirm(umi, { send: { skipPreflight: false } });
}

function sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
} 