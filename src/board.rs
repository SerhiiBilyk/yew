use battleship::field::{status_u8, GameField, Point, ShipDirection, Status};
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use super::js_rand;

pub struct Board {
    board: GameField,
}
pub enum Msg {
    DoIt,
}


impl Component for Board {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let board = GameField::new(js_rand);
        Board { board }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DoIt => {
                self.board.generate_random_field();   
                true
            }
        }
    }
}

impl Renderable<Board> for Board {
    fn view(&self) -> Html<Self> {
        let columns = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        html! {
            // Render your Board here
            <div>
            <div>
                {for columns.iter().map(render_index)}
            </div>
            <div>
             { for self.board.field.iter().enumerate().map(view_field) }
                <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
             </div>
             </div>

        }
    }
}
fn render_row(elem: &Status) -> Html<Board> {
    let css = {
        match elem {
            Status::Ship => String::from("ship"),
            Status::Bound => String::from("bound"),
            _ => String::from("empty"),
        }
    };
    html! {
        <span class=format!("{} {}",String::from("cell"),css),>
           {status_u8(*elem)}
        </span>
    }
}
fn render_index(index: &i32) -> Html<Board> {
    html! {
        <span class="cell coordinates",>{index}</span>
    }
}

fn view_field((index, elem): (usize, &[Status; 12])) -> Html<Board> {
    html! {
          <div>
               <span class="cell coordinates",>{index+1}</span> {for elem.iter().map(render_row)}
            </div>
    }
}
