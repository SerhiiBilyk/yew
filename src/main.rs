use battleship::field::Point;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::services::ConsoleService;


mod board;
use board::Board;

#[macro_use]
extern crate stdweb;
use stdweb::unstable::TryInto;

pub fn js_rand(bottom: u8, top: u8) -> u8 {
    let rand = js! { return Math.random(); };
    let base: f64 = rand.try_into().unwrap();
    (base * (top - 1) as f64).floor() as u8 + bottom
}


struct Model {
    console: ConsoleService,
}

enum Msg {
    DoIt,
}

impl Component for Model {
    // Some details omitted. Explore the examples to see more.

    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {

        Model {
            console: ConsoleService::new(),

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                let number = js_rand(10, 100).to_string();
                let console = self.console.log(&number);


                true
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {

        html! {
            <div>
                <Board: />
                <Board: />
             </div>
        }
    }
}

fn main() {

    yew::start_app::<Model>();
}
