use std::fmt;

pub struct TxHash(pub [u8; 64]);

impl fmt::Debug for TxHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

pub struct Address(pub [u8; 40]);

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

pub struct HexBytes(pub Vec<u8>);

impl fmt::Debug for HexBytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl From<&[u8]> for HexBytes {
    fn from(v: &[u8]) -> HexBytes {
        HexBytes(v.to_vec())
    }
}

impl PartialEq for TxHash {
    fn eq(&self, other: &Self) -> bool{
        return self.0 == other.0
    }
}

#[derive(Debug)]
pub struct EvmTxReceipt {
    pub tx_hash: Vec<u8>,

}

#[derive(Debug)]
pub struct EvmTransaction {
    pub tx_hash: TxHash,
    pub timestamp: u64,
    pub block_number: u64,
    pub from_address: Address,
    pub to_address: Address,
    pub value: u64,
    pub gas: u64,
    pub gas_used: u64,
    pub nonce: u64,
    pub input_data: HexBytes,
}

impl PartialEq for EvmTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.tx_hash == other.tx_hash
    }
}

pub struct EvmInternalTransaction {
    pub parent_tx_hash: TxHash,
}

pub struct EthereumTxReceiptLog {
    pub log_index: u64,
    pub data: HexBytes,
    pub address: Address,
    pub removed: bool,
    pub topics: Vec<HexBytes>,
}

pub struct EthereumTxReceipt {
    pub tx_hash: u64,
    pub contract_address: Address,
    pub status: bool,
    pub tx_type: u64,
    pub logs: Vec<EthereumTxReceiptLog>,
}