pub const BACKEND: &'static str = "http://localhost:8000";

pub mod login;
pub mod route;

fn main() {
    yew::Renderer::<route::Main>::new().render();
}
