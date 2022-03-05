use wordol_solver::{IdolName, IncludedInUnit, IsEligible};

fn main() {
    let mut unit_list = wordol_solver::read_unit_data();

    let output = [
        IncludedInUnit::SameType(IdolName::Tamaki),
        IncludedInUnit::SameType(IdolName::Umi),
        IncludedInUnit::Included(IdolName::Mirai),
        IncludedInUnit::None(IdolName::Tsubasa),
        IncludedInUnit::SameType(IdolName::Empty),
    ];

    unit_list.retain(|_, unit| unit.is_eligible(&output));

    println!("{:?}", unit_list);
}
