import assert from 'assert';
import { AMM } from '../src/amm';

describe('AMM', () => {
    test('test_buy_and_sell_too_much', () => {
        const curve = new AMM(
            30_000_000_000n, // virtualSolReserves
            1_073_000_000_000_000n, // virtualTokenReserves
            0n, // realSolReserves
            793_100_000_000_000n // realTokenReserves
        );
        const curveInitial = { ...curve };

        // Attempt to buy more tokens than available in reserves
        const buyResult = curve.applyBuy(2_000_000_000_000_000_000n);
        console.log("buyResult", buyResult);

        assert(buyResult !== null);
        assert.equal(buyResult.token_amount, 793_100_000_000_000n); // Max amount in curve
        assert.equal(buyResult.sol_amount, 85_005_359_057n); // Should be max cost of curve
        assert.equal(
            curve.realTokenReserves,
            curveInitial.realTokenReserves - buyResult.token_amount
        );
        assert.equal(
            curve.virtualTokenReserves,
            curveInitial.virtualTokenReserves - buyResult.token_amount
        );
        assert.equal(
            curve.realSolReserves,
            curveInitial.realSolReserves + buyResult.sol_amount
        );
        assert.equal(
            curve.virtualSolReserves,
            curveInitial.virtualSolReserves + buyResult.sol_amount
        );

        // Attempt to sell more tokens than available in reserves
        const sellResult = curve.applySell(1_000_000_000_000_000n);
        assert(sellResult === null);
    });

    test('test_apply_sell', () => {
        const curve = new AMM(
            30_000_000_000n, // virtualSolReserves
            1_073_000_000_000_000n, // virtualTokenReserves
            0n, // realSolReserves
            793_100_000_000_000n // realTokenReserves
        );

        // First apply buy
        curve.applyBuy(1_000_000_000n); // 1 SOL
        const curveInitial = { ...curve };
        const sellAmount = 34_612_903_225_806n; // Exact tokens received from buy

        const result = curve.applySell(sellAmount);
        console.log("sellResult", result);

        assert(result !== null);
        assert.equal(result.token_amount, sellAmount);
        assert.equal(result.sol_amount, 1_000_000_000n); // Manually assert
        assert.equal(
            curve.virtualTokenReserves,
            curveInitial.virtualTokenReserves + result.token_amount
        );
        assert.equal(
            curve.realTokenReserves,
            curveInitial.realTokenReserves + result.token_amount
        );
        assert.equal(
            curve.virtualSolReserves,
            curveInitial.virtualSolReserves - result.sol_amount
        );
        assert.equal(
            curve.realSolReserves,
            curveInitial.realSolReserves - result.sol_amount
        );
    });

    test('test_get_sol_for_sell_tokens', () => {
        const curve = new AMM(
            30_000_000_000n, // virtualSolReserves
            1_073_000_000_000_000n, // virtualTokenReserves
            0n, // realSolReserves
            793_100_000_000_000n // realTokenReserves
        );

        // First apply 1 SOL buy
        const buyResult = curve.applyBuy(1_000_000_000n);
        console.log("buyResult", buyResult);

        // Edge case: zero tokens
        assert.equal(curve.getSolForSellTokens(0n), null);

        // Normal case
        assert.equal(
            curve.getSolForSellTokens(34_612_904_000_000n),
            1_000_000_022n // Slightly higher due to bonding curve behavior
        );
    });

    test('test_get_tokens_for_buy_sol', () => {
        const curve = new AMM(
            30_000_000_000n, // virtualSolReserves
            1_073_000_000_000_000n, // virtualTokenReserves
            0n, // realSolReserves
            793_100_000_000_000n // realTokenReserves
        );

        // Test case 1: Normal case 0.01 SOL
        assert.equal(curve.getTokensForBuySol(10_000_000n), 357_547_484_171n);

        // Test case 2: Buy of 1 SOL
        assert.equal(
            curve.getTokensForBuySol(1_000_000_000n),
            34_612_903_225_806n
        );

        // Test case 3: Edge case - zero SOL
        assert.equal(curve.getTokensForBuySol(0n), null);

        // Test case 4: Large SOL amount (but within limits) - 50 SOL
        assert.equal(
            curve.getTokensForBuySol(50_000_000_000n),
            670_625_000_000_000n // 670M token about 66% of total supply
        );
    });
});