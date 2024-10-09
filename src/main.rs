use yew::prelude::*;

mod education;
mod experience;
mod projects;
mod skills;

use education::*;
use experience::*;
use projects::*;
use skills::*;

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
    let interests = ["Robotics"]
        .iter()
        .map(|i| {
            html! {
                <div class={"interest"}> {i} </div>
            }
        })
        .collect::<Vec<_>>();

    html! {
        <section id={"home"}>
            <article>
                <div id={"head"}/>
            </article>

            <article>
                <h1> {"Alan"} </h1>
                <h5>
                    {"Highly motivated and driven by a passion for robotics, AI, and automation R&D."}
                </h5>
                <h5>
                    {"Adept at self-learning and quickly mastering new technologies and skills."}
                </h5>
                <div>
                    <a href={"https://github.com/dizzyi/resume/blob/main/resume.pdf"}> {"CV"} </a>
                </div>
                <h4> {"Research Interests"} </h4>
                <div id={"interests"}>
                    {interests}
                </div>
            </article>

        </section>
    }
}

#[function_component]
fn Contact() -> Html {
    let clipboard = yew_hooks::use_clipboard();

    let email_cb = Callback::from(move |_e| {
        clipboard.write_text("deeralan827@gmail.com".to_string());
    });

    html! {
        <section id={"contact"}>
            <h2>
            {"Contacts"}
            </h2>
            <article>
                <div class={"contact"}>
                    <a href={"mailto: deeralan827@gmail.com"}> {"deeralan827@gmail"} </a>
                    <a onclick={email_cb} id={"email"}>
                    </a>
                </div>
            </article>
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
            <script>
                {include_str!("../index.js")}
            </script>
            <Nav/>
            <Home/>
            <Experience/>
            <Projects/>
            <Education/>
            <Skills/>
            <Contact/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
