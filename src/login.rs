use crate::{
    route::{AuthenticationCtx, Route},
    BACKEND,
};
use aftblog_common::auth::{AuthToken, Claim, LoginResponse};
use gloo::{net::http::Request, timers::future::TimeoutFuture};
use wasm_bindgen_futures::spawn_local;
use web_sys::{FocusEvent, HtmlInputElement, MouseEvent};
use yew::prelude::*;
use yew::{props, suspense::use_future};
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Pass {
    pass: String,
}

#[function_component]
pub fn Login() -> Html {
    let pass_ref = use_node_ref();
    let render = use_state(|| 0);

    let onclick = {
        let render = render.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            render.set(*render + 1);
        })
    };
    let onsubmit = {
        let render = render.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            render.set(*render + 1);
        })
    };

    let pass = pass_ref.cast::<HtmlInputElement>();
    let pass = if pass.is_none() {
        "".to_owned()
    } else {
        pass.unwrap().value()
    };
    let pass = props! { Pass { pass }};

    html! {
        <main>
            <form class="login" onsubmit={onsubmit}>
                <input type="password" id="pass" name="Password" placeholder="Password..." autofocus=true required=true ref={pass_ref} />
                <input type="button" id="login" value="Login" onclick={onclick} />
                if *render > 0 {
                    <Suspense fallback={{html! {"Loading..."}}}>
                        <LoginResult key={*render} ..pass />
                    </Suspense>
                }
            </form>
        </main>
    }
}

async fn reauthenticate(jwt: &str) -> Option<(AuthToken, Claim)> {
    let req = Request::post(&format!("{}/api/auth/renew", BACKEND))
        .header("Authorization", &format!("Bearer {}", jwt));
    let resp = req.send().await;
    if resp.is_err() {
        return None;
    }
    let resp = resp.unwrap();
    if resp.status() != 200 {
        return None;
    }

    let resp = resp.text().await;
    if resp.is_err() {
        return None;
    }

    let resp = ron::from_str::<LoginResponse>(&resp.unwrap());
    if resp.is_err() {
        return None;
    }

    if let LoginResponse::Success(jwt, claim) = resp.unwrap() {
        Some((jwt, claim))
    } else {
        None
    }
}

#[function_component]
pub fn Reauth() -> Html {
    let ctx = use_context::<AuthenticationCtx>().expect("No context found");
    use_effect(move || {
        spawn_local(async move {
            TimeoutFuture::new(60 * 1000).await;
            let res = if let Some(ref jwt) = ctx.jwt {
                reauthenticate(&format!("{}", jwt)).await
            } else {
                None
            };
            if let Some(res) = res {
                ctx.dispatch(res);
            }
        });

        || {}
    });

    html! {}
}

async fn login_request(pass: &str, ctx: AuthenticationCtx) -> bool {
    let req = Request::post(&format!("{}/api/auth/login", BACKEND)).body(pass);
    let resp = req.send().await;
    if resp.is_err() {
        return false;
    }
    let resp = resp.unwrap();
    if resp.status() != 200 {
        return false;
    }

    let resp = resp.text().await;
    if resp.is_err() {
        return false;
    }

    let resp = ron::from_str::<LoginResponse>(&resp.unwrap());
    if resp.is_err() {
        return false;
    }

    let resp = resp.unwrap();

    if let LoginResponse::Success(jwt, claim) = resp {
        ctx.dispatch((jwt, claim));
        return true;
    }

    false
}

#[function_component]
pub fn LoginResult(props: &Pass) -> HtmlResult {
    let pass = props.pass.clone();
    let ctx = use_context::<AuthenticationCtx>().expect("No context found");
    let res = use_future(|| async move { login_request(&pass, ctx).await })?;

    let nav = use_navigator().unwrap();
    if *res {
        nav.push(&Route::Home);
    }

    Ok(html! {
        <p class="loginfail">{"Incorrect password!"}</p>
    })
}
