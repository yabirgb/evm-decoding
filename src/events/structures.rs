use primitive_types::U256;

use crate::transactions::structures::{Address, TxHash};

#[derive(Debug)]
pub enum EventType {
    Trade,
    Staking,
    Deposit,
    Withdrawal,
    Receive,
    Adjustment,
    Unknown,
    Informational,
    Migrate,
    Renew,
    Spend,
}

#[derive(Debug)]
pub enum EventSubType {
    Reward,
    DepositAsset,
    RemoveAsset,
    Fee,
    Spend,
    Receive,
    Deploy,
    Airdrop,
    Bridge,
    Governance,
    None,
    GenerateDebt,
    PaybackDebt,
    ReceiveWrapped,
    Donate,
    Nft,
    PlaceOrder,
    Liquidate,
}

#[derive(Debug)]
pub struct BaseEntry {
    pub event_identfiier: TxHash,
    pub sequence_index: u128,
    pub timestamp: u64,
    pub event_type: EventType,
    pub asset: Address,
    pub amount: U256,
    pub location_label: String,
    pub counterparty: String,
    pub extra_data: Option<String>,
}
