use crate::route::Route;
use common::posts::PostResponse;
use gloo::net::http::Request;
use web_sys::MouseEvent;
use yew::prelude::*;
use yew::suspense::use_future;
use yew_router::prelude::*;

#[function_component]
pub fn Home() -> Html {
    let nav = use_navigator().expect("No router");
    let onclick = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        nav.push(&Route::New);
    });

    // TODO: toggle between projects and posts
    html! {
        <main id="home">
            <button id="new_post" {onclick}>
                <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 4v16m8-8H4" />
                </svg>
            </button>
            <hr />
            <Suspense fallback={{html!{ <ul id="posts"></ul>}}}>
                <Posts />
            </Suspense>
        </main>
    }
}

async fn get_posts() -> Result<Vec<PostResponse>, ()> {
    let req = Request::get(&format!("{}/api/post/get", crate::BACKEND));
    let resp = req.send().await;

    if resp.is_err() {
        return Err(());
    }

    let resp = resp.unwrap();
    if resp.status() != 200 {
        return Err(());
    }

    let resp = resp.text().await;
    if resp.is_err() {
        return Err(());
    }

    let resp = ron::from_str::<Option<Vec<PostResponse>>>(&resp.unwrap());
    if let Ok(resp) = resp {
        if let Some(resp) = resp {
            Ok(resp)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

#[function_component]
fn Posts() -> HtmlResult {
    let res = use_future(|| async move { get_posts().await })?;

    if let Ok(res) = &*res {
        Ok(html! {
            <ul id={"posts"}>
                { for res.into_iter().enumerate().map(|(id, post)| {
                    html! { <li key={id}><PostView content={post.clone()} /></li> }
                }) }
                <li>{"Test content"}</li>
                <li>{"Test content"}</li>
                <li>{"Test content"}</li>
            </ul>
        })
    } else {
        return Ok(html! {
            <ul id={"posts"}>
            </ul>
        });
    }
}

#[derive(Properties, PartialEq)]
struct ViewProps {
    pub content: PostResponse,
}

#[function_component]
fn PostView(props: &ViewProps) -> Html {
    // TODO: anchor tags
    html! {
        <>
            <h3>{ props.content.title.clone() }</h3>
            <input type="checkbox" class="published_marker" name={format!("post-{}", props.content.id)}/>
            <label for={format!("post-{}", props.content.id)} />
        </>
    }
}
