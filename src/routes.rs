use crate::components::*;
//use yew::html::ChildrenProps;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    ComingSoon,
    #[at("/terminal")]
    Terminal,
    #[at("/home")]
    Home,
    #[at("/about")]
    About,
    #[at("/experience")]
    Experience,
    #[at("/projects")]
    Projects,
    #[at("/stack")]
    Stack,
    #[at("/games")]
    Games,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// If Coming Soon show that page otherwise use the layout with whatever page was routed to
pub fn switch(route: Route) -> Html {
    match route {
        Route::ComingSoon => html! { <ComingSoon/> },
        _ => html! { <Layout>{ render_page(route) }</Layout> },
    }
}

fn render_page(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home/> },
        Route::About => html! { <h1>{ "About" }</h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        _ => html! { <Redirect<Route> to={Route::NotFound}/> },
    }
}

// Dont understand
#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
fn layout(props: &LayoutProps) -> Html {
    html! {
        <>
            <NavBar />
            { for props.children.iter() }
        </>
    }
}
