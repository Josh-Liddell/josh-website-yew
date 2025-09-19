use crate::components::*;
use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html!(
        <main class="home">
            //<h1>{ "A Home Component" }</h1>
            <div class="cards-section">
                <IntroCard/>
                <SmallCards/>
                //<BigCards/>
                //<ProjectCards/>
            </div>
            //<Quotes/>
        </main>
    )
}
