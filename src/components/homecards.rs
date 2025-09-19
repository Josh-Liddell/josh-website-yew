use yew::prelude::*;

#[function_component(IntroCard)]
pub fn introcard() -> Html {
    html!(
        //<div class="into-container">
        <div class="intro-card">
            <h1>{ "➥ Hi, I'm Joshua" }</h1>
            <p>{ "I am a software developer and tech enthusiast" }</p>
            <p>{ "Check out my projects, and explore!" }</p>
            <div class="buttons">
                <button class="btn1">{ "About" }</button>
                <button class="btn1">{ "Contact" }</button>
            </div>
        </div>
            //<video class="intro-video">
                //<source src="video.mp4" type="video/mp4"/>
            //</video>
        //</div>
    )
}

//FIX: Map data from the data folder
#[function_component(SmallCard)]
fn smallcard() -> Html {
    html!(
        <div class="small-card">
            <p>{ "Item" }</p>
            <p>{ "Item" }</p>
            <p>{ "Item" }</p>
        </div>
    )
}

#[function_component(BigCard)]
fn bigcard() -> Html {
    // use like a enum & match for the type maybe?
    html!(
        <div class="big-card1">
            <p>{ "Big Item" }</p>
        </div>
    )
}

#[function_component(SmallCards)]
pub fn smallcards() -> Html {
    // Later for each item in the small card data, map it to a smallcard (pass in arguments)

    html!(
        <div class="small-cards">
            <SmallCard/>
            <SmallCard/>
            <SmallCard/>
            <SmallCard/>
        </div>
    )
}

#[function_component(BigCards)]
pub fn bigcards() -> Html {
    // Later for each item in the small card data, map it to a smallcard (pass in arguments)

    html!(
        <div class="big-cards">
            <BigCard/>
            <BigCard/>
        </div>
    )
}
