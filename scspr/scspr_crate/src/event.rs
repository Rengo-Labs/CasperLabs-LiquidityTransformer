use alloc::string::{String, ToString};
use casper_types::{Key, U256};

pub enum SCSPREvent {
    DepositedLiquidity {
        deposit_amount: U256,
        transformer_address: Key,
    },
    Withdrawal {
        from_address: Key,
        token_amount: U256,
    },
    FormedLiquidity {
        cover_amount: U256,
        amount_token_a: U256,
        amount_token_b: U256,
        liquidity: U256,
    },
}

impl SCSPREvent {
    pub fn type_name(&self) -> String {
        match self {
            SCSPREvent::DepositedLiquidity {
                deposit_amount: _,
                transformer_address: _,
            } => "depositedLiquidity",
            SCSPREvent::Withdrawal {
                from_address: _,
                token_amount: _,
            } => "withdrawal",
            SCSPREvent::FormedLiquidity {
                cover_amount: _,
                amount_token_a: _,
                amount_token_b: _,
                liquidity: _,
            } => "formedLiquidityv",
        }
        .to_string()
    }
}
