/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { PublicKey } from '@metaplex-foundation/umi';
import { Serializer, bool, publicKey as publicKeySerializer, struct, u64, u8 } from '@metaplex-foundation/umi/serializers';


export type GlobalSettingsInput = { initialVirtualTokenReserves: bigint; initialVirtualSolReserves: bigint; initialRealTokenReserves: bigint; tokenTotalSupply: bigint; mintDecimals: number; migrateFeeAmount: bigint; migrationTokenAllocation: bigint; feeReceiver: PublicKey; whitelistEnabled: boolean; meteoraConfig: PublicKey;  };

export type GlobalSettingsInputArgs = { initialVirtualTokenReserves: number | bigint; initialVirtualSolReserves: number | bigint; initialRealTokenReserves: number | bigint; tokenTotalSupply: number | bigint; mintDecimals: number; migrateFeeAmount: number | bigint; migrationTokenAllocation: number | bigint; feeReceiver: PublicKey; whitelistEnabled: boolean; meteoraConfig: PublicKey;  };


export function getGlobalSettingsInputSerializer(): Serializer<GlobalSettingsInputArgs, GlobalSettingsInput> {
  return struct<GlobalSettingsInput>([['initialVirtualTokenReserves', u64()], ['initialVirtualSolReserves', u64()], ['initialRealTokenReserves', u64()], ['tokenTotalSupply', u64()], ['mintDecimals', u8()], ['migrateFeeAmount', u64()], ['migrationTokenAllocation', u64()], ['feeReceiver', publicKeySerializer()], ['whitelistEnabled', bool()], ['meteoraConfig', publicKeySerializer()]], { description: 'GlobalSettingsInput' }) as Serializer<GlobalSettingsInputArgs, GlobalSettingsInput>;
}


