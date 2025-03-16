use std::fmt::Display;

use alloy::primitives::{Address, address};
use rand::seq::IteratorRandom;
use strum::{EnumIter, IntoEnumIterator};

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq, EnumIter, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    MON,
    USDC,
    WMON,
    SHMON,
    APRMON,
}

impl Token {
    pub const fn decimals(&self) -> u8 {
        match self {
            Token::MON => 18,
            Token::USDC => 6,
            Token::WMON => 18,
            Token::SHMON => 18,
            Token::APRMON => 18,
        }
    }

    pub const fn address(&self) -> Address {
        match self {
            Token::MON => Address::ZERO,
            Token::USDC => address!("0xf817257fed379853cde0fa4f97ab987181b1e5ea"),
            Token::WMON => address!("0x760AfE86e5de5fa0Ee542fc7B7B713e1c5425701"),
            Token::SHMON => address!("0x3a98250F98Dd388C211206983453837C8365BDc1"),
            Token::APRMON => address!("0xb2f82D0f38dc453D596Ad40A37799446Cc89274A"),
        }
    }

    pub const fn is_native(&self) -> bool {
        match self {
            Token::MON => true,
            Token::USDC => false,
            Token::WMON => false,
            Token::SHMON => false,
            Token::APRMON => false,
        }
    }

    pub const fn is_swap_allowed(&self) -> bool {
        match self {
            Token::MON => true,
            Token::USDC => true,
            Token::WMON => false,
            Token::SHMON => false,
            Token::APRMON => false,
        }
    }

    pub fn ticker(&self) -> &'static str {
        match self {
            Token::MON => "MON",
            Token::USDC => "USDC",
            Token::WMON => "wMON",
            Token::SHMON => "shMON",
            Token::APRMON => "aprMON",
        }
    }

    pub fn random_excluding(exclude: Token) -> Token {
        let mut rng = rand::rng();
        Token::iter().filter(|&t| t != exclude && t.is_swap_allowed()).choose(&mut rng).unwrap() // unless there are at least 2 enum variants the unwrap is safe
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ticker())
    }
}
