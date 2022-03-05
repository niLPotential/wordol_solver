use std::collections::HashSet;
use wordol_solver::*;

#[test]
fn test_enum_idol() {
    let unit_list = read_unit_data();
    assert_eq!(
        unit_list["Legend Days"],
        HashSet::from([
            IdolName::Hibiki,
            IdolName::Yayoi,
            IdolName::Iori,
            IdolName::Ami,
            IdolName::Ritsuko
        ])
    );

    assert!(unit_list["Legend Days"].contains(&IdolName::Hibiki));
}

#[test]
fn test_is_eligible() {
    let unit_list = read_unit_data();

    assert!(unit_list["FAIRY STARS"].is_eligible(&[
        IncludedInUnit::None(IdolName::Sayoko),
        IncludedInUnit::None(IdolName::Kotoha),
        IncludedInUnit::SameType(IdolName::Ayumu),
        IncludedInUnit::None(IdolName::Konomi),
        IncludedInUnit::None(IdolName::Reika),
    ]));

    assert!(unit_list["Kawaranai Mono"].is_eligible(&[
        IncludedInUnit::Included(IdolName::Sayoko),
        IncludedInUnit::None(IdolName::Tomoka),
        IncludedInUnit::None(IdolName::Chizuru),
        IncludedInUnit::None(IdolName::Subaru),
        IncludedInUnit::SameType(IdolName::Miki),
    ]));

    assert!(unit_list["Himitsu no Memories"].is_eligible(&[
        IncludedInUnit::SameType(IdolName::Momoko),
        IncludedInUnit::None(IdolName::Konomi),
        IncludedInUnit::Included(IdolName::Fuka),
        IncludedInUnit::SameType(IdolName::Empty),
        IncludedInUnit::SameType(IdolName::Empty)
    ]));
}
