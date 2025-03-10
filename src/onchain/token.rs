use std::fmt::Display;

use alloy::primitives::{Address, address};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    USDC,
    MON,
    WMON,
    SHMON,
}

impl Token {
    pub const fn decimals(&self) -> u8 {
        match self {
            Token::USDC => 6,
            Token::MON => 18,
            Token::WMON => 18,
            Token::SHMON => 18,
        }
    }

    pub const fn address(&self) -> Address {
        match self {
            Token::USDC => address!("0xf817257fed379853cde0fa4f97ab987181b1e5ea"),
            Token::MON => Address::ZERO,
            Token::WMON => address!("0x760AfE86e5de5fa0Ee542fc7B7B713e1c5425701"),
            Token::SHMON => address!("0x3a98250F98Dd388C211206983453837C8365BDc1"),
        }
    }

    pub const fn is_native(&self) -> bool {
        match self {
            Token::USDC => false,
            Token::MON => true,
            Token::WMON => false,
            Token::SHMON => false,
        }
    }

    pub fn ticker(&self) -> &'static str {
        match self {
            Token::USDC => "USDC",
            Token::MON => "MON",
            Token::WMON => "wMON",
            Token::SHMON => "shMON",
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ticker())
    }
}
