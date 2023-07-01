use leptos::*;
use leptos_router::*;

#[server(OneSecondFn "/api")]
async fn one_second_fn(cx: RequestScope) -> Result<String, ServerFnError> {
    use actix_web::dev::ConnectionInfo;
    use leptos_actix::extract;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    Ok(extract(
        cx,
        |info: ConnectionInfo| async move { format!("{info:?}") },
    )
    .await?)
}

#[server(TwoSecondFn "/api")]
async fn two_second_fn(cx: RequestScope) -> Result<String, ServerFnError> {
    use actix_web::dev::ConnectionInfo;
    use leptos_actix::extract;

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    Ok(extract(
        cx,
        |info: ConnectionInfo| async move { format!("{info:?}") },
    )
    .await?)
}

#[component]
pub fn App() -> impl IntoView {
    let style = r#"
        nav {
            display: flex;
            width: 100%;
            justify-content: space-around;
        }

        [aria-current] {
            font-weight: bold;
        }
    "#;
    view! {

        <style>{style}</style>
        <Router>
            <nav>
                <A href="/out-of-order">"Out-of-Order"</A>
                <A href="/in-order">"In-Order"</A>
                <A href="/async">"Async"</A>
            </nav>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|| view! {  <Redirect path="/out-of-order"/> }
                    />
                    // out-of-order
                    <Route
                        path="out-of-order"
                        view=|| view! {
                            <SecondaryNav/>
                            <h1>"Out-of-Order"</h1>
                            <Outlet/>
                        }
                    >
                        <Route path="" view=|| view! {  <Nested/> }/>
                        <Route path="inside" view=|| view! { <NestedResourceInside/> }/>
                        <Route path="single" view=|| view! {  <Single/> }/>
                        <Route path="parallel" view=|| view! {  <Parallel/> }/>
                        <Route path="inside-component" view=|| view! {  <InsideComponent/> }/>
                        <Route path="none" view=|| view! { <None/> }/>
                    </Route>
                    // in-order
                    <Route
                        path="in-order"
                        ssr=SsrMode::InOrder
                        view=|| view! {
                            <SecondaryNav/>
                            <h1>"In-Order"</h1>
                            <Outlet/>
                        }
                    >
                        <Route path="" view=|| view! {  <Nested/> }/>
                        <Route path="inside" view=|| view! { <NestedResourceInside/> }/>
                        <Route path="single" view=|| view! {  <Single/> }/>
                        <Route path="parallel" view=|| view! {  <Parallel/> }/>
                        <Route path="inside-component" view=|| view! {  <InsideComponent/> }/>
                        <Route path="none" view=|| view! { <None/> }/>
                    </Route>
                    // async
                    <Route
                        path="async"
                        ssr=SsrMode::Async
                        view=|| view! {
                            <SecondaryNav/>
                            <h1>"Async"</h1>
                            <Outlet/>
                        }
                    >
                        <Route path="" view=|| view! {  <Nested/> }/>
                        <Route path="inside" view=|| view! { <NestedResourceInside/> }/>
                        <Route path="single" view=|| view! {  <Single/> }/>
                        <Route path="parallel" view=|| view! {  <Parallel/> }/>
                        <Route path="inside-component" view=|| view! {  <InsideComponent/> }/>
                        <Route path="none" view=|| view! { <None/> }/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn SecondaryNav() -> impl IntoView {
    view! {
        <nav>
            <A href="" exact=true>"Nested"</A>
            <A href="inside" exact=true>"Nested (resource created inside)"</A>
            <A href="single">"Single"</A>
            <A href="parallel">"Parallel"</A>
            <A href="inside-component">"Inside Component"</A>
            <A href="none">"No Resources"</A>
        </nav>
    }
}

#[component]
fn Nested() -> impl IntoView {
    let one_second =
        create_resource(|| (), |_| one_second_fn(Default::default()));
    let two_second =
        create_resource(|| (), |_| two_second_fn(Default::default()));
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <Suspense fallback=|| "Loading 1...">
                "One Second: "
                {move || {
                    one_second.read().map(|_| "Loaded 1!")
                }}
                <br/><br/>
                <Suspense fallback=|| "Loading 2...">
                    "Two Second: "
                    {move || {
                        two_second.read().map(|_| view! {
                            "Loaded 2!"
                            <button on:click=move |_| set_count.update(|n| *n += 1)>
                                {count}
                            </button>
                        })
                    }}
                </Suspense>
            </Suspense>
        </div>
    }
}

#[component]
fn NestedResourceInside() -> impl IntoView {
    let one_second =
        create_resource(|| (), |_| one_second_fn(Default::default()));
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <Suspense fallback=|| "Loading 1...">
                "One Second: "
                {move || {
                    one_second.read().map(|_| {
                        let two_second = create_resource(|| (), move |_| async move {
                            leptos::log!("creating two_second resource");
                            two_second_fn(Default::default()).await
                        });
                        view! {
                            <p>{move || one_second.read().map(|_| "Loaded 1!")}</p>
                            <Suspense fallback=|| "Loading 2...">
                                "Two Second: "
                                {move || {
                                    two_second.read().map(|x| view! {
                                        "Loaded 2 (created inside first suspense)!: "
                                        {format!("{x:?}")}
                                        <button on:click=move |_| set_count.update(|n| *n += 1)>
                                            {count}
                                        </button>
                                    })
                                }}
                            </Suspense>
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn Parallel() -> impl IntoView {
    let one_second =
        create_resource(|| (), |_| one_second_fn(Default::default()));
    let two_second =
        create_resource(|| (), |_| two_second_fn(Default::default()));
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <Suspense fallback=|| "Loading 1...">
                "One Second: "
                {move || {
                    one_second.read().map(move |_| view! {
                        "Loaded 1"
                        <button on:click=move |_| set_count.update(|n| *n += 1)>
                            {count}
                        </button>
                    })
                }}
            </Suspense>
            <br/><br/>
            <Suspense fallback=|| "Loading 2...">
                "Two Second: "
                {move || {
                    two_second.read().map(move |_| view! {
                        "Loaded 2"
                        <button on:click=move |_| set_count.update(|n| *n += 1)>
                            {count}
                        </button>
                    })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn Single() -> impl IntoView {
    let one_second =
        create_resource(|| (), |_| one_second_fn(Default::default()));
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <Suspense fallback=|| "Loading 1...">
                "One Second: "
                {move || {
                    one_second.read().map(|_| "Loaded 1!")
                }}
            </Suspense>
            <p>"Children following " <code>"<Suspense/>"</code> " should hydrate properly."</p>
            <div>
                <button on:click=move |_| set_count.update(|n| *n += 1)>
                    {count}
                </button>
            </div>
        </div>
    }
}

#[component]
fn InsideComponent() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <p><code>"<Suspense/>"</code> " inside another component should work."</p>
            <InsideComponentChild/>
            <p>"Children following " <code>"<Suspense/>"</code> " should hydrate properly."</p>
            <div>
                <button on:click=move |_| set_count.update(|n| *n += 1)>
                    {count}
                </button>
            </div>
        </div>
    }
}

#[component]
fn InsideComponentChild() -> impl IntoView {
    let one_second =
        create_resource(|| (), |_| one_second_fn(Default::default()));
    view! {
        <Suspense fallback=|| "Loading 1...">
            "One Second: "
            {move || {
                one_second.read().map(|_| "Loaded 1!")
            }}
        </Suspense>
    }
}

#[component]
fn None() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div>
            <Suspense fallback=|| "Loading 1...">
                <div>"Children inside Suspense should hydrate properly."</div>
                <button on:click=move |_| set_count.update(|n| *n += 1)>
                    {count}
                </button>
            </Suspense>
            <p>"Children following " <code>"<Suspense/>"</code> " should hydrate properly."</p>
            <div>
                <button on:click=move |_| set_count.update(|n| *n += 1)>
                    {count}
                </button>
            </div>
        </div>
    }
}
