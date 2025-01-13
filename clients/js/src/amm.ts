import { BondingCurve } from "./generated";

export type BuyResult = {
    token_amount: bigint;
    sol_amount: bigint;
};

export type SellResult = {
    token_amount: bigint;
    sol_amount: bigint;
};

export class AMM {
    constructor(
        public virtualSolReserves: bigint,
        public virtualTokenReserves: bigint,
        public realSolReserves: bigint,
        public realTokenReserves: bigint,
    ) { }

    static fromBondingCurve(bondingCurve: BondingCurve): AMM {
        return new AMM(
            bondingCurve.virtualSolReserves,
            bondingCurve.virtualTokenReserves,
            bondingCurve.realSolReserves,
            bondingCurve.realTokenReserves,
        );
    }

    getTokensForBuySol(solAmount: bigint): bigint | null {
        if (solAmount === 0n) return null;

        // Convert to common decimal basis (using 9 decimals as base)
        const currentSol = this.virtualSolReserves;
        const currentTokens = (this.virtualTokenReserves * 1_000_000_000n) / 1_000_000n; // Scale to 9 decimals

        // Calculate new reserves using constant product formula
        const newSol = currentSol + solAmount;
        const newTokens = (currentSol * currentTokens) / newSol;
        const tokensOut = currentTokens - newTokens;

        // Convert back to 6 decimal places for tokens
        return (tokensOut * 1_000_000n) / 1_000_000_000n;
    }

    getSolForSellTokens(tokenAmount: bigint): bigint | null {
        if (tokenAmount === 0n) return null;

        // Convert to common decimal basis (using 9 decimals as base)
        const currentSol = this.virtualSolReserves;
        const currentTokens = (this.virtualTokenReserves * 1_000_000_000n) / 1_000_000n; // Scale to 9 decimals

        // Calculate new reserves using constant product formula
        const newTokens = currentTokens + ((tokenAmount * 1_000_000_000n) / 1_000_000n);
        const newSol = (currentSol * currentTokens) / newTokens;

        return currentSol - newSol;
    }

    applyBuy(solAmount: bigint): BuyResult | null {
        let tokenAmount = this.getTokensForBuySol(solAmount);
        if (!tokenAmount) return null;

        if (tokenAmount >= this.realTokenReserves) {
            // Last Buy
            tokenAmount = this.realTokenReserves;

            // Store current state
            const currentVirtualTokenReserves = this.virtualTokenReserves;
            const currentVirtualSolReserves = this.virtualSolReserves;

            // Update virtual reserves
            this.virtualTokenReserves = this.virtualTokenReserves - tokenAmount;
            this.virtualSolReserves = 115_005_359_056n; // Total raise amount at end

            const recomputedSolAmount = this.getSolForSellTokens(tokenAmount);
            if (recomputedSolAmount) {
                solAmount = recomputedSolAmount;
            }

            // Restore state
            this.virtualTokenReserves = currentVirtualTokenReserves;
            this.virtualSolReserves = currentVirtualSolReserves;
        }

        // Update reserves
        this.virtualTokenReserves = this.virtualTokenReserves - tokenAmount;
        this.realTokenReserves = this.realTokenReserves - tokenAmount;
        this.virtualSolReserves = this.virtualSolReserves + solAmount;
        this.realSolReserves = this.realSolReserves + solAmount;

        return {
            token_amount: tokenAmount,
            sol_amount: solAmount
        };
    }

    applySell(tokenAmount: bigint): SellResult | null {
        if (tokenAmount > this.realTokenReserves) return null;

        const solAmount = this.getSolForSellTokens(tokenAmount);
        if (!solAmount) return null;

        // Update reserves
        this.virtualTokenReserves = this.virtualTokenReserves + tokenAmount;
        this.realTokenReserves = this.realTokenReserves + tokenAmount;
        this.virtualSolReserves = this.virtualSolReserves - solAmount;
        this.realSolReserves = this.realSolReserves - solAmount;

        return {
            token_amount: tokenAmount,
            sol_amount: solAmount
        };
    }

    getPrice(): number {
        // Convert to common decimal basis (using 9 decimals as base)
        const currentSol = Number(this.virtualSolReserves);
        const currentTokens = Number(this.virtualTokenReserves) * 1000000000 / 1000000; // Scale to 9 decimals

        // Price = SOL / Token
        return currentSol / currentTokens;
    }

    getMarketCap(): number {
        // MarketCap = Total Supply * Price
        const price = this.getPrice();
        return 1000000000 * price;
    }
}
