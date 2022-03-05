use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash)]
enum IdolType {
    Princess,
    Fairy,
    Angel,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Hash, Clone, Copy)]
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

impl IdolName {
    fn idoltype(&self) -> IdolType {
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

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy, Deserialize)]
pub enum IncludedInUnit {
    Included(IdolName),
    SameType(IdolName),
    None(IdolName),
}

trait TypeMap {
    fn typemap(&self) -> HashMap<IdolType, usize>;
}

type Unit = HashSet<IdolName>;

impl TypeMap for Unit {
    fn typemap(&self) -> HashMap<IdolType, usize> {
        let mut output = HashMap::new();

        self.iter().for_each(|idol| {
            *output.entry(idol.idoltype()).or_insert(0) += 1;
        });

        output.insert(IdolType::Empty, 5 - self.len());

        output
    }
}

pub trait IsEligible {
    fn is_eligible(&self, output: &WordolOutput) -> bool;
}

// TODO: Fix this
impl IsEligible for Unit {
    fn is_eligible(&self, output: &WordolOutput) -> bool {
        let mut unit_typemap = self.typemap();

        for result in output {
            if let IncludedInUnit::Included(idol) = result {
                if self.contains(idol) {
                    *unit_typemap.entry(idol.idoltype()).or_insert(0) -= 1;
                } else {
                    return false;
                }
            }
        }

        for result in output {
            if let IncludedInUnit::SameType(idol) = result {
                let counter = unit_typemap.entry(idol.idoltype()).or_insert(0);
                if *counter == 0 || self.contains(idol) {
                    return false;
                } else {
                    *counter -= 1;
                }
            }
        }

        for result in output {
            if let IncludedInUnit::None(idol) = result {
                if *unit_typemap.entry(idol.idoltype()).or_insert(0) != 0 || self.contains(idol) {
                    return false;
                }
            }
        }

        true
    }
}

type WordolOutput = [IncludedInUnit; 5];

pub fn read_unit_data() -> HashMap<String, Unit> {
    let unit_data = fs::read_to_string("units.json").expect("Unable to read unit data");
    serde_json::from_str(&unit_data).expect("Unable to parse json into HashNap<String, Unit>")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idoltype_trait() {
        let azusa = IdolName::Azusa;
        assert_eq!(IdolType::Angel, azusa.idoltype());
        let chihaya = IdolName::Chihaya;
        assert_eq!(IdolType::Fairy, chihaya.idoltype());
        let mirai = IdolName::Mirai;
        assert_eq!(IdolType::Princess, mirai.idoltype());
    }

    #[test]
    fn test_typemap() {
        let unit_list = read_unit_data();
        assert_eq!(
            unit_list["Kawaranai Mono"].typemap(),
            HashMap::from([
                (IdolType::Princess, 2),
                (IdolType::Angel, 2),
                (IdolType::Empty, 1)
            ])
        );
    }

    #[test]
    #[ignore]
    fn test_optimal() {
        let unit_list = read_unit_data();

        let maybe_optimal = [
            IdolName::Umi,
            IdolName::Mizuki,
            IdolName::Tsubasa,
            IdolName::Mirai,
            IdolName::Serika,
        ];

        let mut output_set = HashSet::new();

        for (name, unit) in &unit_list {
            assert!(
                output_set.insert(generate_output(unit, &maybe_optimal)),
                "Duplicate Found: {}:{:?}",
                name,
                unit
            );
        }

        assert_eq!(unit_list.len(), output_set.len());
    }

    trait New {
        fn new() -> Self;
    }

    impl New for WordolOutput {
        fn new() -> Self {
            [IncludedInUnit::None(IdolName::Empty); 5]
        }
    }

    fn generate_output(unit: &Unit, input: &[IdolName; 5]) -> WordolOutput {
        let mut output = WordolOutput::new();
        let mut not_included = Vec::new();
        let mut unit_typemap = unit.typemap();

        for i in 0..5 {
            if unit.contains(&input[i]) {
                output[i] = IncludedInUnit::Included(input[i]);
                *unit_typemap.entry(input[i].idoltype()).or_insert(0) -= 1;
            } else {
                not_included.push(i);
            }
        }

        for i in not_included {
            let counter = unit_typemap.entry(input[i].idoltype()).or_insert(0);
            if *counter > 0 {
                output[i] = IncludedInUnit::SameType(input[i]);
                *counter -= 1;
            } else {
                output[i] = IncludedInUnit::None(input[i]);
            }
        }

        output
    }
}
