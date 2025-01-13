/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Context, Pda, PublicKey, Signer, TransactionBuilder, publicKey, transactionBuilder } from '@metaplex-foundation/umi';
import { Serializer, array, mapSerializer, struct, u8 } from '@metaplex-foundation/umi/serializers';
import { ResolvedAccount, ResolvedAccountsWithIndices, getAccountMetasAndSigners } from '../shared';

// Accounts.
export type CreatePoolInstructionAccounts = {
    global: PublicKey | Pda;
    bondingCurve: PublicKey | Pda;
    feeReceiver: PublicKey | Pda;
    pool: PublicKey | Pda;
    config: PublicKey | Pda;
    lpMint: PublicKey | Pda;
    aVaultLp: PublicKey | Pda;
    bVaultLp: PublicKey | Pda;
    tokenAMint: PublicKey | Pda;
    tokenBMint: PublicKey | Pda;
    aVault: PublicKey | Pda;
    bVault: PublicKey | Pda;
    aTokenVault: PublicKey | Pda;
    bTokenVault: PublicKey | Pda;
    aVaultLpMint: PublicKey | Pda;
    bVaultLpMint: PublicKey | Pda;
    bondingCurveTokenAccount: PublicKey | Pda;
    feeReceiverTokenAccount: PublicKey | Pda;
    bondingCurveSolEscrow: PublicKey | Pda;
    payerTokenA: PublicKey | Pda;
    payerTokenB: PublicKey | Pda;
    payerPoolLp: PublicKey | Pda;
    protocolTokenAFee: PublicKey | Pda;
    protocolTokenBFee: PublicKey | Pda;
    payer?: Signer;
    mintMetadata: PublicKey | Pda;
    rent?: PublicKey | Pda;
    metadataProgram: PublicKey | Pda;
    vaultProgram: PublicKey | Pda;
    tokenProgram?: PublicKey | Pda;
    associatedTokenProgram: PublicKey | Pda;
    systemProgram?: PublicKey | Pda;
    meteoraProgram: PublicKey | Pda;
};

  // Data.
  export type CreatePoolInstructionData = { discriminator: Array<number>;  };

export type CreatePoolInstructionDataArgs = {  };


  export function getCreatePoolInstructionDataSerializer(): Serializer<CreatePoolInstructionDataArgs, CreatePoolInstructionData> {
  return mapSerializer<CreatePoolInstructionDataArgs, any, CreatePoolInstructionData>(struct<CreatePoolInstructionData>([['discriminator', array(u8(), { size: 8 })]], { description: 'CreatePoolInstructionData' }), (value) => ({ ...value, discriminator: [233, 146, 209, 142, 207, 104, 64, 188] }) ) as Serializer<CreatePoolInstructionDataArgs, CreatePoolInstructionData>;
}




// Instruction.
export function createPool(
  context: Pick<Context, "payer" | "programs">,
                        input: CreatePoolInstructionAccounts,
      ): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey('pumpScience', 'EtZR9gh25YUM6LkL2o2yYV1KzyuDdftHvYk3wsb2Ypkj');

  // Accounts.
  const resolvedAccounts = {
          global: { index: 0, isWritable: true as boolean, value: input.global ?? null },
          bondingCurve: { index: 1, isWritable: true as boolean, value: input.bondingCurve ?? null },
          feeReceiver: { index: 2, isWritable: true as boolean, value: input.feeReceiver ?? null },
          pool: { index: 3, isWritable: true as boolean, value: input.pool ?? null },
          config: { index: 4, isWritable: false as boolean, value: input.config ?? null },
          lpMint: { index: 5, isWritable: true as boolean, value: input.lpMint ?? null },
          aVaultLp: { index: 6, isWritable: true as boolean, value: input.aVaultLp ?? null },
          bVaultLp: { index: 7, isWritable: true as boolean, value: input.bVaultLp ?? null },
          tokenAMint: { index: 8, isWritable: false as boolean, value: input.tokenAMint ?? null },
          tokenBMint: { index: 9, isWritable: true as boolean, value: input.tokenBMint ?? null },
          aVault: { index: 10, isWritable: true as boolean, value: input.aVault ?? null },
          bVault: { index: 11, isWritable: true as boolean, value: input.bVault ?? null },
          aTokenVault: { index: 12, isWritable: true as boolean, value: input.aTokenVault ?? null },
          bTokenVault: { index: 13, isWritable: true as boolean, value: input.bTokenVault ?? null },
          aVaultLpMint: { index: 14, isWritable: true as boolean, value: input.aVaultLpMint ?? null },
          bVaultLpMint: { index: 15, isWritable: true as boolean, value: input.bVaultLpMint ?? null },
          bondingCurveTokenAccount: { index: 16, isWritable: true as boolean, value: input.bondingCurveTokenAccount ?? null },
          feeReceiverTokenAccount: { index: 17, isWritable: true as boolean, value: input.feeReceiverTokenAccount ?? null },
          bondingCurveSolEscrow: { index: 18, isWritable: true as boolean, value: input.bondingCurveSolEscrow ?? null },
          payerTokenA: { index: 19, isWritable: true as boolean, value: input.payerTokenA ?? null },
          payerTokenB: { index: 20, isWritable: true as boolean, value: input.payerTokenB ?? null },
          payerPoolLp: { index: 21, isWritable: true as boolean, value: input.payerPoolLp ?? null },
          protocolTokenAFee: { index: 22, isWritable: true as boolean, value: input.protocolTokenAFee ?? null },
          protocolTokenBFee: { index: 23, isWritable: true as boolean, value: input.protocolTokenBFee ?? null },
          payer: { index: 24, isWritable: true as boolean, value: input.payer ?? null },
          mintMetadata: { index: 25, isWritable: true as boolean, value: input.mintMetadata ?? null },
          rent: { index: 26, isWritable: false as boolean, value: input.rent ?? null },
          metadataProgram: { index: 27, isWritable: false as boolean, value: input.metadataProgram ?? null },
          vaultProgram: { index: 28, isWritable: false as boolean, value: input.vaultProgram ?? null },
          tokenProgram: { index: 29, isWritable: false as boolean, value: input.tokenProgram ?? null },
          associatedTokenProgram: { index: 30, isWritable: false as boolean, value: input.associatedTokenProgram ?? null },
          systemProgram: { index: 31, isWritable: false as boolean, value: input.systemProgram ?? null },
          meteoraProgram: { index: 32, isWritable: true as boolean, value: input.meteoraProgram ?? null },
      } satisfies ResolvedAccountsWithIndices;

  
    // Default values.
  if (!resolvedAccounts.payer.value) {
        resolvedAccounts.payer.value = context.payer;
      }
      if (!resolvedAccounts.rent.value) {
        resolvedAccounts.rent.value = publicKey('SysvarRent111111111111111111111111111111111');
      }
      if (!resolvedAccounts.tokenProgram.value) {
        resolvedAccounts.tokenProgram.value = context.programs.getPublicKey('splToken', 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');
resolvedAccounts.tokenProgram.isWritable = false
      }
      if (!resolvedAccounts.systemProgram.value) {
        resolvedAccounts.systemProgram.value = context.programs.getPublicKey('splSystem', '11111111111111111111111111111111');
resolvedAccounts.systemProgram.isWritable = false
      }
      
  // Accounts in order.
      const orderedAccounts: ResolvedAccount[] = Object.values(resolvedAccounts).sort((a,b) => a.index - b.index);
  
  
  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(orderedAccounts, "programId", programId);

  // Data.
      const data = getCreatePoolInstructionDataSerializer().serialize({});
  
  // Bytes Created On Chain.
      const bytesCreatedOnChain = 0;
  
  return transactionBuilder([{ instruction: { keys, programId, data }, signers, bytesCreatedOnChain }]);
}