use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::{chat::ChatPage, not_found::NotFound};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Chat"/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=ChatPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
