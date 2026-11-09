use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes,A},
    StaticSegment,
};
use leptos_struct_table::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
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
        <Stylesheet id="leptos" href="/pkg/ru_leptos3.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
                <nav style="padding: 0.5rem; border-bottom: 1px solid #ddd">
                    <A href="/">"Home"</A>
                    " | "
                    <A href="/about">"About"</A>
                    " | "
                    <A href="/todos">"Todos"</A>
                </nav>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("")       view=HomePage/>
                    <Route path=StaticSegment("/about") view=AboutPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    use leptos::logging::log;
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| { 
        *count.write() += 1;
        log!("Button clicked! Count is now: {}", *count.read());
    };

    view! {
        <h1>"Home Page!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[derive(TableRow, Clone)]
#[table(impl_vec_data_provider)]
pub struct Person {
    id: u32,
    name: String,
    age: u32,
}

#[component]
fn AboutPage() -> impl IntoView {
    let rows = vec![
        Person { id: 1, name: "John".to_string(), age: 32 },
        Person { id: 2, name: "Jane".to_string(), age: 28 },
        Person { id: 3, name: "Bob".to_string(), age: 45 },
        Person { id: 4, name: "Alice".to_string(), age: 22 },
        Person { id: 5, name: "Charlie".to_string(), age: 37 },
    ];

    view! {
        <h4>"Persons"</h4>
        <table>
            <TableContent rows scroll_container="" />
        </table>
    }
}