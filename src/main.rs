pub const BACKEND: &str = "http://localhost:8000";

pub mod home;
pub mod login;
pub mod missing;
pub mod new;
pub mod route;

fn main() {
    yew::Renderer::<route::Main>::new().render();
}
