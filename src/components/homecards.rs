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

#[function_component(BigCard1)]
fn bigcard1() -> Html {
    html!(
        <div class="big-card1">
            <p>{ "Big Item 1" }</p>
        </div>
    )
}

#[function_component(BigCard2)]
fn bigcard2() -> Html {
    html!(
        <a class="big-card2">
            <div class="inner-card2">
                <p>{ "Big Item 2" }</p>
            </div>
        </a>
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
            <BigCard1/>
            <BigCard2/>
        </div>
    )
}

#[function_component(ProjectCard)]
fn projectcard() -> Html {
    html!(
        <div class="project-card">
            <p>{ "Item" }</p>
        </div>
    )
}

#[function_component(ProjectCards)]
pub fn projectcards() -> Html {
    // Later for each item in the small card data, map it to a smallcard (pass in arguments)

    html!(
        <div class="project-cards">
            <ProjectCard/>
            <ProjectCard/>
            <ProjectCard/>
            <ProjectCard/>
            <ProjectCard/>
            <ProjectCard/>
            <ProjectCard/>
            <ProjectCard/>
            <ProjectCard/>
            <ProjectCard/>
            <ProjectCard/>
            <ProjectCard/>
        </div>
    )
}

#[function_component(Quotes)]
pub fn quotes() -> Html {
    // Later for each item in the small card data, map it to a smallcard (pass in arguments)

    html!(
        <div class="quotes">
            <p>{ "Item" }</p>
        </div>
    )
}
