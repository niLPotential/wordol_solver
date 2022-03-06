use std::collections::HashSet;
use wordol_solver::{IdolName, IncludedInUnit, IsEligible, SingleOutput};

#[test]
fn test_retained_unit_list() {
    let mut unit_list = wordol_solver::read_unit_data();

    let sample_output = [
        SingleOutput {
            idolname: IdolName::Momoko,
            included_in_unit: IncludedInUnit::Included,
        },
        SingleOutput {
            idolname: IdolName::Tsubasa,
            included_in_unit: IncludedInUnit::None,
        },
        SingleOutput {
            idolname: IdolName::Serika,
            included_in_unit: IncludedInUnit::None,
        },
        SingleOutput {
            idolname: IdolName::Mirai,
            included_in_unit: IncludedInUnit::None,
        },
        SingleOutput {
            idolname: IdolName::Umi,
            included_in_unit: IncludedInUnit::None,
        },
    ];

    unit_list.retain(|_, unit| unit.is_eligible(&sample_output));

    assert_eq!(
        HashSet::from(["Cut. Cut. Cut.".to_string(), "Jelly PoP Beans".to_string()]),
        unit_list.into_keys().collect()
    );
}
