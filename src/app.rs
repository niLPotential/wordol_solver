use enum_iterator::IntoEnumIterator;
use num_traits::FromPrimitive;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement};
use wordol_solver::{IdolName, IncludedInUnit, SingleOutput, WordolOutput};
use yew::{events::Event, html, Component, Context, Html};

pub enum Msg {
    IdolName(IdolName),
    IncludedInUnit(IncludedInUnit),
}

pub struct Model {
    value: WordolOutput,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: [SingleOutput {
                idolname: IdolName::Empty,
                included_in_unit: IncludedInUnit::None,
            }; 5],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::IdolName(idolname) => {
                self.value[0].idolname = idolname;
                true
            }
            Msg::IncludedInUnit(included_in_unit) => {
                self.value[0].included_in_unit = included_in_unit;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

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

        html! {
            <div>
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

                <p>{self.value[0].idolname}</p>
                <p>{self.value[0].included_in_unit}</p>


            </div>
        }
    }
}
