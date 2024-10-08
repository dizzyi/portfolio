use yew::prelude::*;

mod education;
mod experience;

use education::*;
use experience::*;

#[function_component]
fn Nav() -> Html {
    let tabs = [
        "Home",
        "Experience",
        "Projects",
        "Education",
        "Skills",
        "Contact",
    ]
    .into_iter()
    .map(|s| {
        html! {
            <li>
                <a href={format!("#{}",s.to_lowercase())} > {s} </a>
            </li>
        }
    })
    .collect::<Vec<_>>();

    html! {
        <nav>
            <ul class={"card"}>{tabs}</ul>
        </nav>
    }
}

#[function_component]
fn Home() -> Html {
    html! {
        <section id={"home"}>
            {"Home"}
        </section>
    }
}

#[function_component]
fn Projects() -> Html {
    html! {
        <section id={"projects"}>
            {"Projects"}
        </section>
    }
}

#[function_component]
fn Skills() -> Html {
    html! {
        <section id={"skills"}>
            {"Skills"}
        </section>
    }
}

#[function_component]
fn Contact() -> Html {
    html! {
        <section id={"contact"}>
            {"Contacts"}
        </section>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <style>
                {include_str!("../index.css")}
            </style>
            <Nav/>
            // <Home/>
            // <Experience/>
            // <Projects/>
            <Education/>
            // <Skills/>
            // <Contact/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
