use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
struct ButtonProps {
    #[prop_or(Route::Home)]
    link: Route,
    #[prop_or_default]
    text: String,
}

#[function_component]
fn Button(props: &ButtonProps) -> Html {
    let navigator = use_navigator().expect("No nav");
    let goto = props.link;

    let onclick = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        navigator.push(&goto);
    });

    html! {
        <button onclick={onclick}>{props.text.clone()}</button>
    }
}

#[function_component]
pub fn Home() -> Html {
    html! {
        <main id="home">
            <Button text={"test"} link={Route::New} />
        </main>
    }
}
