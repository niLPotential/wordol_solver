use enum_iterator::IntoEnumIterator;
use num_derive::FromPrimitive;
use serde::Deserialize;
use std::fmt;
use std::hash::Hash;

pub const UNIT_LIST: &str = include_str!("../units.json");

#[derive(Eq, PartialEq, Hash)]
pub enum IdolType {
    Princess,
    Fairy,
    Angel,
    Empty,
}

#[derive(Deserialize, Hash, PartialEq, Eq, Debug, Copy, Clone, IntoEnumIterator, FromPrimitive)]
pub enum IdolName {
    Empty,
    Haruka,
    Chihaya,
    Miki,
    Yukiho,
    Yayoi,
    Makoto,
    Iori,
    Takane,
    Ritsuko,
    Azusa,
    Ami,
    Mami,
    Hibiki,
    Mirai,
    Shizuka,
    Tsubasa,
    Kotoha,
    Elena,
    Minako,
    Megumi,
    Matsuri,
    Serika,
    Akane,
    Anna,
    Roco,
    Yuriko,
    Sayoko,
    Arisa,
    Umi,
    Iku,
    Tomoka,
    Emily,
    Shiho,
    Ayumu,
    Hinata,
    Kana,
    Nao,
    Chizuru,
    Konomi,
    Tamaki,
    Fuka,
    Miya,
    Noriko,
    Mizuki,
    Karen,
    Rio,
    Subaru,
    Reika,
    Momoko,
    Julia,
    Tsumugi,
    Kaori,
}

impl fmt::Display for IdolName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IdolName {
    pub fn idoltype(&self) -> IdolType {
        match &self {
            Self::Empty => IdolType::Empty,
            Self::Haruka
            | Self::Yukiho
            | Self::Makoto
            | Self::Hibiki
            | Self::Mirai
            | Self::Kotoha
            | Self::Minako
            | Self::Matsuri
            | Self::Yuriko
            | Self::Sayoko
            | Self::Arisa
            | Self::Umi
            | Self::Iku
            | Self::Emily
            | Self::Kana
            | Self::Nao
            | Self::Noriko => IdolType::Princess,
            Self::Chihaya
            | Self::Iori
            | Self::Takane
            | Self::Ritsuko
            | Self::Shizuka
            | Self::Megumi
            | Self::Roco
            | Self::Tomoka
            | Self::Shiho
            | Self::Ayumu
            | Self::Chizuru
            | Self::Mizuki
            | Self::Rio
            | Self::Subaru
            | Self::Momoko
            | Self::Julia
            | Self::Tsumugi => IdolType::Fairy,
            Self::Miki
            | Self::Yayoi
            | Self::Azusa
            | Self::Ami
            | Self::Mami
            | Self::Tsubasa
            | Self::Elena
            | Self::Serika
            | Self::Akane
            | Self::Anna
            | Self::Hinata
            | Self::Konomi
            | Self::Tamaki
            | Self::Fuka
            | Self::Miya
            | Self::Karen
            | Self::Reika
            | Self::Kaori => IdolType::Angel,
        }
    }
}

#[derive(Deserialize, PartialEq, Copy, Clone, Debug, FromPrimitive)]
pub enum IncludedInUnit {
    None,
    SameType,
    Included,
}

impl fmt::Display for IncludedInUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
