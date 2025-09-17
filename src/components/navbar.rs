use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar">
            <h3>{ "Joshua Liddell" }</h3>
            <ul>
                <li><Link<Route> to={Route::About}>{ "About" }</Link<Route>></li>
                <li><Link<Route> to={Route::Experience}>{ "Experience" }</Link<Route>></li>
                <li><Link<Route> to={Route::Projects}>{ "Projects" }</Link<Route>></li>
                <li><Link<Route> to={Route::Stack}>{ "Stack" }</Link<Route>></li>
                <li><Link<Route> to={Route::Games}>{ "Games" }</Link<Route>></li>
            </ul>
        </nav>
    }
}
