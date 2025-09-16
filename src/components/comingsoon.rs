use yew::prelude::*;

#[function_component(ComingSoon)]
pub fn comingsoon() -> Html {
    html!(
        <div class="coming-soon">
            <div class="titles">
                <h1>{ "Website in Development" }</h1>
                <h2 class="gradient">{ "Come back soon" }</h2>
            </div>
            <footer>
                //<p onClick={onContinue}>joshliddell.com</p>
                <p>{ "joshliddell.com" }</p>
            </footer>
        </div>
    )
}
