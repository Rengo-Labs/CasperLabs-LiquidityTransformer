use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum SyntheticTokenEvent {
    LiquidityRemoved {
        amount_wcspr: U256,
        amount_scspr: U256,
    },
    SendFeesToMaster {
        amount_wcspr: U256,
        master_address: Key,
    },
    LiquidityAdded {
        amount_wcspr: U256,
        amount_scspr: U256,
        liquidity: U256,
    },
    MasterProfit {
        amount_wcspr: U256,
        master_address: Key,
    },
    SendArbitrageProfitToMaster {
        amount_wcspr: U256,
        master_address: Key,
    },
}

impl SyntheticTokenEvent {
    pub fn type_name(&self) -> String {
        match self {
            SyntheticTokenEvent::LiquidityRemoved {
                amount_wcspr: _,
                amount_scspr: _,
            } => "LiquidityRemoved",
            SyntheticTokenEvent::SendFeesToMaster {
                amount_wcspr: _,
                master_address: _,
            } => "SendFeesToMaster",
            SyntheticTokenEvent::LiquidityAdded {
                amount_wcspr: _,
                amount_scspr: _,
                liquidity: _,
            } => "LiquidityAdded",
            SyntheticTokenEvent::MasterProfit {
                amount_wcspr: _,
                master_address: _,
            } => "MasterProfit",
            SyntheticTokenEvent::SendArbitrageProfitToMaster {
                amount_wcspr: _,
                master_address: _,
            } => "SendArbitrageProfitToMaster",
        }
        .to_string()
    }
}
