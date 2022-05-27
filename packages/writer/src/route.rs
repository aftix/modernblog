use crate::home::Home;
use crate::login::{Login, Reauth};
use crate::missing::NotFound;
use crate::new::NewPost;
use common::auth::{AuthToken, Claim};
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Authentication {
    pub(crate) jwt: Option<AuthToken>,
    pub(crate) claim: Option<Claim>,
}

impl Authentication {
    pub fn new() -> Self {
        Self {
            jwt: None,
            claim: None,
        }
    }
}

impl Reducible for Authentication {
    type Action = (AuthToken, Claim);

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Authentication {
            jwt: Some(action.0),
            claim: Some(action.1),
        }
        .into()
    }
}

pub type AuthenticationCtx = UseReducerHandle<Authentication>;

#[derive(Clone, Routable, PartialEq, Copy)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/home")]
    Home,
    #[at("/new")]
    New,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Login => html! { <Login /> },
        Route::NotFound => html! { <NotFound /> },
        Route::Home => html! { <Home /> },
        Route::New => html! { <NewPost />},
    }
}

#[function_component]
pub fn Main() -> Html {
    let auth = use_reducer(Authentication::new);

    html! {
        <ContextProvider<AuthenticationCtx> context={auth}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
                <EnsureLogin />
                <Reauth />
            </BrowserRouter>
        </ContextProvider<AuthenticationCtx>>
    }
}

#[function_component]
pub fn EnsureLogin() -> Html {
    let ctx = use_context::<AuthenticationCtx>().expect("No context found");
    let navigator = use_navigator().unwrap();
    let route: Route = use_route().expect("No route");
    use_effect(move || {
        if ctx.jwt.is_none() && route != Route::Login {
            navigator.push(&Route::Login);
        }
        || {}
    });
    html! {}
}
