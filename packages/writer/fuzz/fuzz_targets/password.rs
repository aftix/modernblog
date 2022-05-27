#![no_main]
use libfuzzer_sys::fuzz_target;

use writer::login::{LoginResult, Pass};
use writer::route::{Authentication, AuthenticationCtx};
use yew::prelude::*;

#[function_component]
fn Test(props: &Pass) -> Html {
    let context = use_reducer(Authentication::new);

    html! {
        <ContextProvider<AuthenticationCtx> {context}>
            <Suspense fallback={html!{""}}>
                <LoginResult pass={props.pass.clone()} />
            </Suspense>
        </ContextProvider<AuthenticationCtx>>
    }
}

fuzz_target!(|data: &[u8]| {
    if let Ok(lossy) = std::str::from_utf8(data) {
        let renderer = yew::ServerRenderer::<Test>::with_props(Pass { pass: lossy.to_string() });

        let _rendered = renderer.render();
    }
});
