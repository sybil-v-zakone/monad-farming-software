use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Dex {
    Ambient,
    Bean,
    Hashflow,
}

impl Display for Dex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Dex::Ambient => "AMBIENT",
            Dex::Bean => "BEAN",
            Dex::Hashflow => "HASHFLOW",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Lending {
    Apriori,
    Kinza,
    Shmonad,
}

impl Display for Lending {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Lending::Apriori => "APRIORI",
            Lending::Kinza => "KINZA",
            Lending::Shmonad => "SHMONAD",
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Nft {
    NadDomains,
}

impl Display for Nft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Nft::NadDomains => "NAD DOMAIN",
        };

        write!(f, "{}", s)
    }
}
