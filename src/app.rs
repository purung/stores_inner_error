use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use reactive_stores::{Field, Store};
use uuid::Uuid;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/storestest.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let base = Store::new(Base::new(vec![Outer::new(
        Uuid::new_v4(),
        vec![Inner::new(Uuid::new_v4())],
    )]));

    let add = move |_| {
        base.outer()
            .write()
            .push(Outer::new(Uuid::new_v4(), vec![Inner::new(Uuid::new_v4())]))
    };
    view! {
        <>
            <For each=move || base.outer() key=|outer| outer.id().get() let:outer>
                <OuterStuff outer />
            </For>
            <button on:click=add>More</button>
        </>
    }
}

#[component]
fn OuterStuff(#[prop(into)] outer: Field<Outer>) -> impl IntoView {
    let add = move |_| outer.inner().write().push(Inner::new(Uuid::new_v4()));
    view! {
        <div style="background:pink;">
            <p>Outer: {outer.read().show()}</p>
            <For each=move || outer.inner() key=|inner| inner.id().get() let:inner>
                <InnerStuff inner />
            </For>
            <button on:click=add>Error</button>
        </div>
    }
}

#[component]
fn InnerStuff(#[prop(into)] inner: Field<Inner>) -> impl IntoView {
    view! { <p>Inner: {inner.read().show()}</p> }
}

#[derive(Debug, Clone, Store)]
struct Base {
    #[store(key: Uuid = |outer| outer.id)]
    outer: Vec<Outer>,
}

impl Base {
    fn new(outer: Vec<Outer>) -> Self {
        Self { outer }
    }
}

#[derive(Debug, Clone, Store)]
struct Outer {
    id: Uuid,
    #[store(key: Uuid = |inner| inner.id)]
    inner: Vec<Inner>,
}

impl Outer {
    fn new(id: Uuid, inner: Vec<Inner>) -> Self {
        Self { id, inner }
    }
    fn show(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Debug, Clone, Store)]
struct Inner {
    id: Uuid,
}

impl Inner {
    fn new(id: Uuid) -> Self {
        Self { id }
    }
    fn show(&self) -> String {
        self.id.to_string()
    }
}
