use crate::login::Login;
use aftblog_common::auth::*;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Authentication {
    pub(crate) jwt: Option<AuthToken>,
    pub(crate) claim: Option<Claim>,
}

impl Authentication {
    pub(crate) fn new() -> Self {
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

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/home")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Login => html! { <Login /> },
        Route::NotFound => html! { "404" },
        Route::Home => html! { "home" },
    }
}

#[function_component]
pub fn Main() -> Html {
    let auth = use_reducer(|| Authentication::new());

    html! {
        <ContextProvider<AuthenticationCtx> context={auth}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<AuthenticationCtx>>
    }
}
