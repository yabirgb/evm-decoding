use crate::{
    events::structures::{BaseEntry, EventType},
    transactions::structures::{Address, EthereumTxReceiptLog, EvmTransaction},
};
use std::collections::HashMap;

trait Decoder {
    fn counterparties(&self) -> Vec<&str>;
    fn addresses_to_decoders(
        &self,
    ) -> HashMap<Address, Vec<Box<dyn Fn(Vec<EthereumTxReceiptLog>)>>>;
}

pub fn decode_simple_transaction(tx: EvmTransaction) -> Vec<BaseEntry> {
    let mut events: Vec<BaseEntry> = Vec::new();

    // decode the gas usage
    events.push(BaseEntry {
        event_identfiier: tx.tx_hash,
        sequence_index: 0,
        timestamp: tx.timestamp,
        event_type: EventType::Spend,
        asset: Address(
            "0f1a748cDF53Bbad378CE2C4429463d01CcE0C3f"
                .as_bytes()
                .try_into()
                .expect("Wrong length"),
        ),
        amount: tx.gas_used.into(),
        location_label: tx.from_address.to_string(),
        counterparty: "gas".to_string(),
        extra_data: None,
    });

    return events;
}
