mod app;

use app::Model;
// use wordol_solver::IsEligible;

fn main() {
    // let mut unit_list = wordol_solver::read_unit_data();

    // let output = wordol_solver::read_wordol_output();

    // unit_list.retain(|_, unit| unit.is_eligible(&output));

    // for unit in unit_list {
    //     println!("{:?}", unit);
    // }

    yew::start_app::<Model>();
}
