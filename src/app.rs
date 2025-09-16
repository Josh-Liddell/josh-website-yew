use crate::components::*;
use yew::prelude::*;
use yew_router::prelude::*;

// Maybe the route is just part of the navbar component
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/comingsoon")]
    ComingSoon,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        //Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Home => html! { <Redirect<Route> to={Route::ComingSoon}/> },
        Route::ComingSoon => html! {
            <ComingSoon />
        },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            //<NavBar />
            <Switch<Route> render={switch} /> // must be child of BrowserRouter
        </BrowserRouter>
    }
}
