use super::*;

// #[test]
// fn simplest_binary_order_matching_test() {
// 	let (mut runtime, root, accounts) = init_runtime_env();

// 	accounts[0].set_allowance(&mut runtime, flux_protocol(), U128(to_dai(30))).expect("allowance couldn't be set");
// 	let tx_res = accounts[0].create_market(&mut runtime, empty_string(), empty_string(), U64(2), outcome_tags(0), categories(), U64(market_end_timestamp_ms()), U128(0), U128(0), "test".to_string()).unwrap();
// 	assert_eq!(tx_res.status, ExecutionStatus::SuccessValue(b"0".to_vec()));

// 	accounts[0].set_allowance(&mut runtime, flux_protocol(), U128(110000)).expect("allowance couldn't be set");
// 	accounts[0].place_order(&mut runtime, U64(0), U64(0), U128(1000), U128(50), None).expect("order placement tx failed unexpectedly");
// 	accounts[0].place_order(&mut runtime, U64(0), U64(1), U128(1000), U128(50), None).expect("order placement tx failed unexpectedly");

// 	let no_share_balance: u128 = accounts[0].get_outcome_share_balance(&mut runtime, accounts[0].get_account_id(), U64(0), U64(0)).into();
// 	let yes_share_balance: u128 = accounts[0].get_outcome_share_balance(&mut runtime, accounts[0].get_account_id(), U64(0), U64(1)).into();
// 	assert_eq!(no_share_balance, 1000);
// 	assert_eq!(yes_share_balance, 1000);
// }

// #[test]
// fn partial_binary_order_matching_test() {
// 	let (mut runtime, root, accounts) = init_runtime_env();
// 	accounts[0].set_allowance(&mut runtime, flux_protocol(), U128(to_dai(30))).expect("allowance couldn't be set");
// 	let tx_res = accounts[0].create_market(&mut runtime, empty_string(), empty_string(), U64(2), outcome_tags(0), categories(), U64(market_end_timestamp_ms()), U128(0), U128(0), "test".to_string()).unwrap();
// 	assert_eq!(tx_res.status, ExecutionStatus::SuccessValue(b"0".to_vec()));

// 	let res = accounts[0].set_allowance(&mut runtime, flux_protocol(), U128(10000000000)).expect("allowance couldn't be set");

	
// 	let res = accounts[0].place_order(&mut runtime, U64(0), U64(1), U128(1000), U128(50), None).expect("order placement tx failed unexpectedly");
// 	let res = accounts[0].place_order(&mut runtime, U64(0), U64(1), U128(1000), U128(50), None).expect("order placement tx failed unexpectedly");
// 	let res = accounts[0].place_order(&mut runtime, U64(0), U64(1), U128(550), U128(50), None).expect("order placement tx failed unexpectedly");
// 	let res = accounts[0].place_order(&mut runtime, U64(0), U64(1), U128(200), U128(50), None).expect("order placement tx failed unexpectedly");

// 	let res = accounts[0].place_order(&mut runtime, U64(0), U64(0), U128(1000), U128(50), None).expect("order placement tx failed unexpectedly");
// 	let res = accounts[0].place_order(&mut runtime, U64(0), U64(0), U128(1555), U128(50), None).expect("order placement tx failed unexpectedly");

// 	let no_share_balance = accounts[0].get_outcome_share_balance(&mut runtime, accounts[0].get_account_id(), U64(0), U64(0));
// 	let yes_share_balance = accounts[0].get_outcome_share_balance(&mut runtime, accounts[0].get_account_id(), U64(0), U64(1));
// 	assert_eq!(no_share_balance, U128(2555));
// 	assert_eq!(yes_share_balance, U128(2555));
// }


#[test]
fn partial_binary_order_matching_test() {
	let (mut runtime, root, accounts) = init_runtime_env();
	accounts[0].set_allowance(&mut runtime, flux_protocol(), U128(to_dai(30))).expect("allowance couldn't be set");
	let tx_res = accounts[0].create_market(&mut runtime, empty_string(), empty_string(), U64(2), outcome_tags(0), categories(), U64(market_end_timestamp_ms()), U128(0), U128(0), "test".to_string()).unwrap();
	assert_eq!(tx_res.status, ExecutionStatus::SuccessValue(b"0".to_vec()));

	let res = accounts[0].set_allowance(&mut runtime, flux_protocol(), U128(9500000000000000000000000)).expect("allowance couldn't be set");

	
	let res = accounts[0].place_order(&mut runtime, U64(0), U64(0), U128(950000000000000000), U128(50), None).expect("order placement tx failed unexpectedly");
	let res = accounts[0].place_order(&mut runtime, U64(0), U64(1), U128(200000000000000000), U128(50), None).expect("order placement tx failed unexpectedly");
	let res = accounts[0].place_order(&mut runtime, U64(0), U64(1), U128(700000000000000000), U128(50), None).expect("order placement tx failed unexpectedly");
	println!("res: {:?}", res);

	let no_share_balance = accounts[0].get_outcome_share_balance(&mut runtime, accounts[0].get_account_id(), U64(0), U64(0));
	let yes_share_balance = accounts[0].get_outcome_share_balance(&mut runtime, accounts[0].get_account_id(), U64(0), U64(1));
}