#[cfg(test)]
mod tests {
    use anchor_lang::prelude::{msg, Clock, Pubkey};
    use once_cell::sync::Lazy;
    use structs::{BondingCurve, CreateBondingCurveParams};

    use crate::{state::bonding_curve::*, util::bps_mul, Global};
    static START_TIME: u64 = 1;
    static CLOCK: Lazy<Clock> = Lazy::new(|| Clock {
        slot: START_TIME.clone(),
        ..Clock::default()
    });

    #[test]
    fn test_buy_and_sell_too_much() {
        let creator = Pubkey::default();
        let mint = Pubkey::default();

        let global = Global::default();

        let params = CreateBondingCurveParams {
            name: "test".to_string(),
            symbol: "test".to_string(),
            uri: "test".to_string(),
            start_slot: Some(1),
        };
        let mut bc = BondingCurve::default();
        let curve = bc.update_from_params(mint, creator, &global, &params, &CLOCK, 0);
        let curve_initial = curve.clone();

        // Attempt to buy more tokens than available in reserves
        let buy_result = curve.apply_buy(2000000000000000000).unwrap();
        println!("{:?} \n", buy_result);

        assert_eq!(buy_result.token_amount, 793100000000000); // Max amount in curve
        assert_eq!(buy_result.sol_amount, 85005359057); // Should be max cost of curve
        assert_eq!(curve.complete, true);
        assert_eq!(
            curve.real_token_reserves,
            curve_initial.real_token_reserves - buy_result.token_amount
        );
        assert_eq!(
            curve.virtual_token_reserves,
            curve_initial.virtual_token_reserves - buy_result.token_amount
        );
        assert_eq!(
            curve.real_sol_reserves,
            curve_initial.real_sol_reserves + buy_result.sol_amount
        );
        assert_eq!(
            curve.virtual_sol_reserves,
            curve_initial.virtual_sol_reserves + buy_result.sol_amount
        );
        println!("{} \n", curve);
        println!("{:?} \n", buy_result);

        // Attempt to sell more tokens than available in reserves
        let sell_result = curve.apply_sell(1000000000000000);
        assert!(sell_result.is_none());
        println!("{} \n", curve);
        println!("{:?} \n", sell_result);
    }

    #[test]
    fn test_calculate_fee() {
        let bonding_curve = BondingCurve::default();

        let mut current_slot = 1;

        let fee = bonding_curve.calculate_fee(1000, current_slot).unwrap();
        assert_eq!(fee, 990); // 99% of 1000

        // Test Phase 2: Slot 150
        current_slot = 150;
        let fee = bonding_curve.calculate_fee(1000, current_slot).unwrap();
        // Calculate expected fee for slot 150
        let expected_fee = bps_mul(917, 1000, 10_000).unwrap();
        assert_eq!(fee, expected_fee);

        // Test Phase 2: Slot 200
        current_slot = 200;
        let fee = bonding_curve.calculate_fee(1000, current_slot).unwrap();
        // Calculate expected fee for slot 200
        let expected_fee = bps_mul(502, 1000, 10_000).unwrap(); // Example calculation
        assert_eq!(fee, expected_fee);

        // Test Phase 2: Slot 250
        current_slot = 250;
        let fee = bonding_curve.calculate_fee(1000, current_slot).unwrap();
        // Calculate expected fee for slot 250
        let expected_fee = bps_mul(87, 1000, 10_000).unwrap(); // Example calculation
        assert_eq!(fee, expected_fee);

        // Test Phase 3: Slot 300
        current_slot = 300;
        let fee = bonding_curve.calculate_fee(1000, current_slot).unwrap();
        assert_eq!(fee, 10); // 1% of 1000
    }

    #[test]
    fn test_apply_sell() {
        let creator = Pubkey::default();
        let mint = Pubkey::default();
        let global = Global::default();

        let params = CreateBondingCurveParams {
            name: "test".to_string(),
            symbol: "test".to_string(),
            uri: "test".to_string(),
            start_slot: Some(START_TIME),
        };
        let mut bc = BondingCurve::default();
        let curve = bc.update_from_params(mint, creator, &global, &params, &CLOCK, 0);

        // first apply buy
        curve.apply_buy(1000000000).unwrap(); // 1 SOL

        let curve_initial: BondingCurve = curve.clone();
        let sell_amount = 34612903225806; // Exact tokens received from buy

        let result = curve.apply_sell(sell_amount).unwrap();
        println!("{:?} \n", result);
        assert_eq!(result.token_amount, sell_amount);
        assert_eq!(result.sol_amount, 1000000000); // Manually assert
        assert_eq!(
            curve.virtual_token_reserves,
            curve_initial.virtual_token_reserves + result.token_amount
        );
        assert_eq!(
            curve.real_token_reserves,
            curve_initial.real_token_reserves + result.token_amount
        );
        assert_eq!(
            curve.virtual_sol_reserves,
            curve_initial.virtual_sol_reserves - result.sol_amount
        );
        assert_eq!(
            curve.real_sol_reserves,
            curve_initial.real_sol_reserves - result.sol_amount
        );
    }

    #[test]
    fn test_apply_buy() {
        let creator = Pubkey::default();
        let mint = Pubkey::default();
        let global = Global::default();

        let params = CreateBondingCurveParams {
            name: "test".to_string(),
            symbol: "test".to_string(),
            uri: "test".to_string(),
            start_slot: Some(START_TIME),
        };
        let mut bc = BondingCurve::default();

        let curve = bc.update_from_params(mint, creator, &global, &params, &CLOCK, 0);
        let curve_initial = curve.clone();

        let purchase_amount = 1000000000; // 1 SOL

        let result = curve.apply_buy(purchase_amount).unwrap();
        println!("{:?} \n", result);
        assert_eq!(result.sol_amount, purchase_amount);
        assert_eq!(result.token_amount, 34612903225806); // Manually assert
        assert_eq!(
            curve.virtual_token_reserves,
            curve_initial.virtual_token_reserves - result.token_amount
        );
        assert_eq!(
            curve.real_token_reserves,
            curve_initial.real_token_reserves - result.token_amount
        );
        assert_eq!(
            curve.virtual_sol_reserves,
            curve_initial.virtual_sol_reserves + purchase_amount
        ); // See the 2000 addtion
        assert_eq!(curve.real_sol_reserves, purchase_amount);

        // Buy of 1 more SOL
        let result = curve.apply_buy(1000000000).unwrap();
        println!("{:?} \n", result);
        assert_eq!(result.token_amount, 32449596774193);

        // Buy of 1 more SOL
        let result = curve.apply_buy(1000000000).unwrap();
        println!("{:?} \n", result);
        assert_eq!(result.token_amount, 30482954545454); // Lower than first purchase

        // Buy of 1 more SOL
        let result = curve.apply_buy(1000000000).unwrap();
        println!("{:?} \n", result);
        assert_eq!(result.token_amount, 28689839572192); // Lower again

        // Complete bonding curve in multiple steps

        let _result = curve.apply_buy(30000000000).unwrap();
        let _result = curve.apply_buy(30000000000).unwrap();
        let _result = curve.apply_buy(21006000000).unwrap();

        // Total buys of 85.006 SOL cause the curve to complete
        assert_eq!(curve.complete, true);
        assert_eq!(curve.real_token_reserves, 0);
        assert_eq!(curve.real_sol_reserves, 85005359057);
    }

    #[test]
    fn test_get_sol_for_sell_tokens() {
        let creator = Pubkey::default();
        let mint = Pubkey::default();
        let global = Global::default();

        let params = CreateBondingCurveParams {
            name: "test".to_string(),
            symbol: "test".to_string(),
            uri: "test".to_string(),
            start_slot: Some(START_TIME),
        };
        let mut bc = BondingCurve::default();
        let curve = bc.update_from_params(mint, creator, &global, &params, &CLOCK, 0);

        // first apply 1 SOL buy
        let buy_result = curve.apply_buy(1000000000).unwrap();
        println!("{:?} \n", buy_result);

        // Edge case: zero tokens
        assert_eq!(curve.get_sol_for_sell_tokens(0), None);

        // Normal case
        assert_eq!(
            curve.get_sol_for_sell_tokens(34612904000000),
            Some(1000000022) // Slightly higher due to bonding curve bahaviour
        );

        let real_sol_reserves = curve.real_sol_reserves;
        msg!("real_sol_reserves: {}", real_sol_reserves);
    }

    #[test]
    fn test_get_tokens_for_buy_sol() {
        let creator = Pubkey::default();
        let mint = Pubkey::default();
        let global = Global::default();

        let params = CreateBondingCurveParams {
            name: "test".to_string(),
            symbol: "test".to_string(),
            uri: "test".to_string(),
            start_slot: Some(START_TIME),
        };
        let mut bc = BondingCurve::default();
        let mut curve = bc.update_from_params(mint, creator, &global, &params, &CLOCK, 0);

        // Test case 1: Normal case 0.01 SOL SOL
        assert_eq!(curve.get_tokens_for_buy_sol(10000000), Some(357547484171));

        // Test case 2: Buy of 1 SOL
        curve = bc.update_from_params(mint, creator, &global, &params, &CLOCK, 0);
        assert_eq!(
            curve.get_tokens_for_buy_sol(1000000000),
            Some(34612903225806)
        );

        // Test case 3: Edge case - zero SOL
        assert_eq!(curve.get_tokens_for_buy_sol(0), None);

        // Test case 5: Large SOL amount (but within limits) - 50 SOL
        curve = bc.update_from_params(mint, creator, &global, &params, &CLOCK, 0);
        assert_eq!(
            curve.get_tokens_for_buy_sol(50000000000),
            Some(670625000000000) // 670M token about 66% of total supply
        );

        // Test case 5: SOL amount that would exceed real token reserves
        // Check is made in apply_buy directly in order to recompute counter asset
        // assert_eq!(
        //     curve.get_tokens_for_buy_sol(1793100000000000),
        //     Some(curve.real_token_reserves)
        // );
    }

    // FUZZ TESTS
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]

        #[test]
        fn fuzz_test_default_alloc_simple_curve_apply_buy(
            sol_amount in 1..u64::MAX,
        ) {
            let creator = Pubkey::default();
            let mint = Pubkey::default();
            let global = Global::default();

            let params = CreateBondingCurveParams {
                name: "test".to_string(),
                symbol: "test".to_string(),
                uri: "test".to_string(),
                start_slot: Some(START_TIME),
            };
            let mut bc = BondingCurve::default();
            let curve = bc.update_from_params(mint,creator, &global, &params, &CLOCK, 0);
            let _curve_initial = curve.clone();

            if let Some(result) = curve.apply_buy(sol_amount) {
                prop_assert!(result.token_amount <= _curve_initial.real_token_reserves, "Token amount bought should not exceed real token reserves");
            }
        }

        #[test]
        fn fuzz_test_default_alloc_simple_curve_apply_sell(
            token_amount in 1..u64::MAX,
            buy_sol_amount in 1..u64::MAX,
        ) {
            let creator = Pubkey::default();
            let mint = Pubkey::default();
            let global = Global::default();

            let params = CreateBondingCurveParams {
                name: "test".to_string(),
                symbol: "test".to_string(),
                uri: "test".to_string(),
                start_slot: Some(START_TIME),
            };
            let mut bc = BondingCurve::default();
            let curve = bc.update_from_params(mint,creator, &global, &params, &CLOCK, 0);
            let buy_result = curve.apply_buy(buy_sol_amount);
            if buy_result.is_none() {
                return Ok(())
            }
            let _curve_after_buy = curve.clone();
            if let Some(result) = curve.apply_sell(token_amount) {
                prop_assert!(result.sol_amount <= _curve_after_buy.real_sol_reserves, "SOL amount to send to seller should not exceed real SOL reserves");
            }
        }
    }
}
