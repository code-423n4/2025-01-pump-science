import { createSignerFromKeypair, Keypair, Pda, PublicKey, TransactionBuilder, Umi } from "@metaplex-foundation/umi";
import { findAssociatedTokenPda, SPL_ASSOCIATED_TOKEN_PROGRAM_ID } from "@metaplex-foundation/mpl-toolbox";
import { publicKey as publicKeySerializer, string } from '@metaplex-foundation/umi/serializers';
import { fromWeb3JsPublicKey } from "@metaplex-foundation/umi-web3js-adapters";
import { SYSVAR_CLOCK_PUBKEY } from "@solana/web3.js";
import { FEE_RECEIVER, tokenMetadataProgramId } from "../constants";
import {
    createBondingCurve,
    CreateBondingCurveInstructionDataArgs,
    fetchBondingCurve,
    findBondingCurvePda,
    PUMP_SCIENCE_PROGRAM_ID,
    swap,
    SwapInstructionArgs
} from "..";
import { PumpScienceSDK } from "./pump-science";
import { findWLPda } from "../utils";

export class CurveSDK {
    PumpScience: PumpScienceSDK;
    umi: Umi;

    mint: PublicKey;
    userTokenAccount: Pda;
    bondingCurvePda: Pda;
    bondingCurveTokenAccount: Pda;
    bondingCurveSolEscrow: Pda;
    whitelistPda: Pda;
    mintMetaPda: Pda;

    fetchData() {
        return fetchBondingCurve(this.umi, this.bondingCurvePda[0]);
    }

    swap(params: {
        direction: "buy" | "sell",
    } & Pick<SwapInstructionArgs, "exactInAmount" | "minOutAmount">) {
        return swap(this.umi, {
            global: this.PumpScience.globalPda[0],
            user: this.umi.identity,
            baseIn: params.direction !== "buy",
            exactInAmount: params.exactInAmount,
            minOutAmount: params.minOutAmount,
            mint: this.mint,
            bondingCurve: this.bondingCurvePda[0],
            bondingCurveTokenAccount: this.bondingCurveTokenAccount[0],
            bondingCurveSolEscrow: this.bondingCurveSolEscrow[0],
            userTokenAccount: this.userTokenAccount[0],
            feeReceiver: FEE_RECEIVER,
            clock: fromWeb3JsPublicKey(SYSVAR_CLOCK_PUBKEY),
            associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
            ...this.PumpScience.evtAuthAccs,
        });
    }

    createBondingCurve(params: CreateBondingCurveInstructionDataArgs, mintKp: Keypair, whitelist: boolean) {
        // check mintKp is this.mint
        console.log("creating mint ===>>>", this.mint.toString());
        console.log("bondingCurveTokenAccount ===>>>", this.bondingCurveTokenAccount[0].toString());
        console.log("bondingCurvePda ===>>>", this.bondingCurvePda[0].toString());

        if (mintKp.publicKey.toString() !== this.mint.toString()) {
            throw new Error("wrong mintKp provided");
        }

        // Create bonding curve
        const createBondingCurveIx = createBondingCurve(this.umi, {
            global: this.PumpScience.globalPda[0],
            creator: this.umi.identity,
            mint: createSignerFromKeypair(this.umi, mintKp),
            bondingCurve: this.bondingCurvePda[0],
            bondingCurveTokenAccount: this.bondingCurveTokenAccount[0],
            bondingCurveSolEscrow: this.bondingCurveSolEscrow,
            metadata: this.mintMetaPda[0],
            ...this.PumpScience.evtAuthAccs,
            associatedTokenProgram: SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
            ...params,
            whitelist: whitelist ? this.whitelistPda[0] : undefined
        });

        console.log("this.whitelistPda[0]---->>>>", this.whitelistPda[0]);

        // Combine all instructions
        return new TransactionBuilder()
            .add(createBondingCurveIx);
    }

    constructor(sdk: PumpScienceSDK, mint: PublicKey) {
        this.PumpScience = sdk;
        this.umi = sdk.umi;
        this.mint = mint;
        this.userTokenAccount = findAssociatedTokenPda(this.umi, {
            mint: this.mint,
            owner: this.umi.identity.publicKey,
        });


        this.bondingCurvePda = findBondingCurvePda(this.umi, {
            mint: this.mint,
        });
        this.bondingCurveTokenAccount = findAssociatedTokenPda(this.umi, {
            mint: this.mint,
            owner: this.bondingCurvePda[0],
        });
        this.bondingCurveSolEscrow = this.umi.eddsa.findPda(PUMP_SCIENCE_PROGRAM_ID, [
            string({ size: 'variable' }).serialize('sol-escrow'),
            publicKeySerializer().serialize(this.mint),
        ]);

        this.mintMetaPda = this.umi.eddsa.findPda(tokenMetadataProgramId, [
            string({ size: 'variable' }).serialize('metadata'),
            publicKeySerializer().serialize(tokenMetadataProgramId),
            publicKeySerializer().serialize(mint),
        ]);

        this.whitelistPda = findWLPda(this.umi, this.umi.identity.publicKey);
    }
}
