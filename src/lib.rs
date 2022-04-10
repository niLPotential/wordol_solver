use serde::Deserialize;
use std::collections::{HashMap, HashSet};

pub mod data;
use data::{IdolName, IdolType, IncludedInUnit, UNIT_LIST};

#[derive(Deserialize, Copy, Clone)]
pub struct SingleOutput {
    pub idolname: IdolName,
    pub included_in_unit: IncludedInUnit,
}

trait TypeMap {
    fn typemap(&self) -> HashMap<IdolType, usize>;
}

pub type Unit = HashSet<IdolName>;

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

        for result_included in output
            .iter()
            .filter(|&result| result.included_in_unit == IncludedInUnit::Included)
        {
            if self.contains(&result_included.idolname) {
                *unit_typemap
                    .entry(result_included.idolname.idoltype())
                    .or_insert(0) -= 1;
            } else {
                return false;
            }
        }

        for result_sametype in output
            .iter()
            .filter(|&result| result.included_in_unit == IncludedInUnit::SameType)
        {
            let counter = unit_typemap
                .entry(result_sametype.idolname.idoltype())
                .or_insert(0);
            if *counter == 0 || self.contains(&result_sametype.idolname) {
                return false;
            } else {
                *counter -= 1;
            }
        }

        for result_none in output
            .iter()
            .filter(|&result| result.included_in_unit == IncludedInUnit::None)
        {
            if *unit_typemap
                .entry(result_none.idolname.idoltype())
                .or_insert(0)
                != 0
                || self.contains(&result_none.idolname)
            {
                return false;
            }
        }

        true
    }
}

pub type WordolOutput = [SingleOutput; 5];

pub fn read_unit_data() -> HashMap<String, Unit> {
    // let unit_data = fs::read_to_string("units.json").expect("Unable to read unit data")
    // serde_json::from_str(&unit_data).expect("Unable to parse json into HashNap<String, Unit>")
    serde_json::from_str(&UNIT_LIST).expect("Unable to parse json into HashNap<String, Unit>")
}

// pub fn read_wordol_output() -> WordolOutput {
//     let wordol_output =
//         fs::read_to_string("wordol_output.json").expect("Unable to read wordol output");

//     serde_json::from_str(&wordol_output).expect("Unable to parse json into WordolOutput")
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_eligible() {
        let unit_list = read_unit_data();

        let read_test_list_eligible =
            fs::read_to_string("tests_eligible.json").expect("Unable to read eligible test list");
        let test_list_eligible: HashMap<String, WordolOutput> =
            serde_json::from_str(&read_test_list_eligible)
                .expect("Unable to parse json into eligible test list");

        for (name, eligible_output) in test_list_eligible {
            assert!(unit_list[&name].is_eligible(&eligible_output));
        }

        let read_test_list_not_eligible = fs::read_to_string("tests_not_eligible.json")
            .expect("Unable to read not eligible test list");
        let test_list_not_eligible: HashMap<String, WordolOutput> =
            serde_json::from_str(&read_test_list_not_eligible)
                .expect("Unable to parse json into not eligible test list");

        for (name, not_eligible_output) in test_list_not_eligible {
            assert!(!unit_list[&name].is_eligible(&not_eligible_output));
        }
    }
}
