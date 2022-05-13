use yew::prelude::*;

pub mod login;

fn main() {
    yew::Renderer::<login::Login>::new().render();
}
