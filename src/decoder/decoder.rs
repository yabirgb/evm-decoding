use std::collections::HashMap;
use crate::transactions::structures::{Address, EthereumTxReceiptLog};

trait Decoder {
    
    fn counterparties(&self) -> Vec<&str>;
    fn addresses_to_decoders(&self) -> HashMap<Address, Vec<Box<dyn Fn(Vec<EthereumTxReceiptLog>)>>>;

}