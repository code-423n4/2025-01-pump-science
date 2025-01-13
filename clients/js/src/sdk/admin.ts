import { SPL_SYSTEM_PROGRAM_ID } from "@metaplex-foundation/mpl-toolbox";
import { Umi } from "@metaplex-foundation/umi";
import { GlobalSettingsInputArgs } from "..";
import { setParams, SetParamsInstructionAccounts } from '../generated/instructions/setParams';
import { initialize, } from '../generated/instructions/initialize';
import { PumpScienceSDK } from "./pump-science";

export type SetParamsInput = GlobalSettingsInputArgs & Partial<SetParamsInstructionAccounts>;

export class AdminSDK {
    PumpScience: PumpScienceSDK;
    umi: Umi;

    constructor(sdk: PumpScienceSDK) {
        this.PumpScience = sdk;
        this.umi = sdk.umi;
    }

    initialize(params: GlobalSettingsInputArgs) {
        const txBuilder = initialize(this.PumpScience.umi, {
            global: this.PumpScience.globalPda[0],
            authority: this.umi.identity,
            params,
            systemProgram: SPL_SYSTEM_PROGRAM_ID,
            ...this.PumpScience.evtAuthAccs,
        });
        return txBuilder;
    }

    setParams(params: SetParamsInput) {
        const { newAuthority, ...ixParams } = params;

        const parsedParams: GlobalSettingsInputArgs = {
            initialVirtualTokenReserves: ixParams.initialVirtualTokenReserves,
            initialVirtualSolReserves: ixParams.initialVirtualSolReserves,
            initialRealTokenReserves: ixParams.initialRealTokenReserves,
            tokenTotalSupply: ixParams.tokenTotalSupply,
            feeReceiver: ixParams.feeReceiver,
            mintDecimals: ixParams.mintDecimals,
            migrateFeeAmount: ixParams.migrateFeeAmount,
            migrationTokenAllocation: ixParams.migrationTokenAllocation,
            whitelistEnabled: ixParams.whitelistEnabled,
            meteoraConfig: ixParams.meteoraConfig
        };

        const txBuilder = setParams(this.PumpScience.umi, {
            global: this.PumpScience.globalPda[0],
            authority: this.umi.identity,
            params: parsedParams,
            newAuthority,
            ...this.PumpScience.evtAuthAccs,
        });
        return txBuilder;
    }
}
