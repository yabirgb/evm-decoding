use ::poseidon::transactions::structures::{EvmTransaction, TxHash, Address, HexBytes};

fn main() {
    let tx = EvmTransaction {
        tx_hash: TxHash("ba9a52a144d4e79580a557160e9f8269d3e5373ce44bce00ebd609754034b7bd".as_bytes().try_into().expect("Wring length")),
        timestamp: 1646375440,
        block_number: 14318825,
        from_address: Address("0f1a748cDF53Bbad378CE2C4429463d01CcE0C3f".as_bytes().try_into().expect("Wring length")),
        to_address: Address("b4EBc2C371182DeEa04B2264B9ff5AC4F0159C69".as_bytes().try_into().expect("Wring length")),
        value: 0,
        gas: 171249,
        gas_used: 171249,
        nonce: 507,
        input_data: HexBytes::from(String::from("b6b55f250000000000000000000000000000000000000000000000312ebe013bcd5d6fed").as_bytes()),
    };

    println!("{:?}", tx);
}
