use enum_iterator::IntoEnumIterator;
use num_traits::FromPrimitive;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement};
use wordol_solver::{IsEligible, SingleOutput, Unit, WordolOutput};
use yew::{events::Event, html, Component, Context, Html};

use wordol_solver::data::{IdolName, IncludedInUnit};

pub enum Msg {
    Index(usize),
    IdolName(IdolName),
    IncludedInUnit(IncludedInUnit),
    AddResult,
    Compute,
}

pub struct Model {
    index: usize,
    idolname: IdolName,
    included_in_unit: IncludedInUnit,
    wordol_output: WordolOutput,
    unit_list: HashMap<String, Unit>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            unit_list: wordol_solver::read_unit_data(),
            index: 0,
            idolname: IdolName::Empty,
            included_in_unit: IncludedInUnit::None,
            wordol_output: [SingleOutput {
                idolname: IdolName::Empty,
                included_in_unit: IncludedInUnit::None,
            }; 5],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Index(idx) => {
                self.index = idx;
                true
            }
            Msg::IdolName(idolname) => {
                self.idolname = idolname;
                true
            }
            Msg::IncludedInUnit(included_in_unit) => {
                self.included_in_unit = included_in_unit;
                true
            }
            Msg::AddResult => {
                let index = self.index;
                let idolname = self.idolname;
                let included_in_unit = self.included_in_unit;

                self.wordol_output[index] = SingleOutput {
                    idolname,
                    included_in_unit,
                };

                true
            }
            Msg::Compute => {
                self.unit_list
                    .retain(|_, unit| unit.is_eligible(&self.wordol_output));
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_index_change = link.callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            input
                .map(|input| Msg::Index(input.value().parse().unwrap()))
                .unwrap()
        });

        let on_idolname_change = link.callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            input
                .map(|input| {
                    Msg::IdolName(FromPrimitive::from_u8(input.value().parse().unwrap()).unwrap())
                })
                .unwrap()
        });

        let on_status_change = link.callback(|e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            input
                .map(|input| {
                    Msg::IncludedInUnit(
                        FromPrimitive::from_u8(input.value().parse().unwrap()).unwrap(),
                    )
                })
                .unwrap()
        });

        let on_add_result = link.callback(|_| Msg::AddResult);

        let on_compute = link.callback(|_| Msg::Compute);

        html! {
            <div>
                <select name="index" onchange={on_index_change}>
                    <option value="0"> {1}</option>
                    <option value="1"> {2}</option>
                    <option value="2"> {3}</option>
                    <option value="3"> {4}</option>
                    <option value="4"> {5}</option>

                </select>

                <select name="idolname" onchange={on_idolname_change}>
                {
                    IdolName::into_enum_iter().map(|idolname|{
                        html!{<option value={format!("{}",idolname as u8)}>{idolname}</option>}
                    }).collect::<Html>()
                }
                </select>

                <select name="status" onchange={on_status_change}>
                    <option value="0" >{"Wrong"}</option>
                    <option value="1" >{"SameType"}</option>
                    <option value="2" >{"Correct"}</option>
                </select>

                <button onclick={on_add_result}>{"submit"}</button>

                <button onclick={on_compute}>{"compute"}</button>

                <p>{self.wordol_output[0].idolname}{" "}{self.wordol_output[0].included_in_unit}</p>
                <p>{self.wordol_output[1].idolname}{" "}{self.wordol_output[1].included_in_unit}</p>
                <p>{self.wordol_output[2].idolname}{" "}{self.wordol_output[2].included_in_unit}</p>
                <p>{self.wordol_output[3].idolname}{" "}{self.wordol_output[3].included_in_unit}</p>
                <p>{self.wordol_output[4].idolname}{" "}{self.wordol_output[4].included_in_unit}</p>

                <>{
                    self.unit_list.iter().map(|unit|{
                        html!{<p>{format!("{:?}", unit)}</p>}
                    }).collect::<Html>()
                }</>

            </div>
        }
    }
}
