use yew::prelude::*;

#[function_component]
pub fn Login() -> Html {
    html! {
        <form class="login">
            <input type="password" id="pass" name="Password" placeholder="Password..." autofocus=true required=true />
            <input type="button" id="login" value="Login" />
        </form>
    }
}
