use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::items::{ItemView, ItemViewProps, ItemsView, ItemsViewProps};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! {cx,
        <>
            <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
            <Stylesheet id="leptos" href="/pkg/leptos_playground.css"/>
            <Meta name="description" content="Leptos playground project."/>
            <Router>
                <nav>
                    <A exact=true href="/">"Home"</A>
                    <A href="items">"Items"</A>
                </nav>
                <main>
                    <Routes>
                        <Route path="/" view=move |cx| view! {cx, <h1>"Home "</h1>}/>
                        <Route path="items" view=move |cx| view! {cx, <div><h1>"Items"</h1><ItemsView/></div>}/>
                        <Route path="items/:id" view=move |cx| view! {cx, <h1>"Item"</h1><ItemView/>}/>
                    </Routes>
                </main>
            </Router>
        </>
    }
}
