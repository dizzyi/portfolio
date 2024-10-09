use crate::*;

#[function_component]
pub fn Skills() -> Html {
    let skills = [
        (
            "Language",
            vec![
                "Native Cantonese",
                "Fluent in English",
                "Fluent in Mandarin",
            ],
            "D32F2F",
        ),
        (
            "Programming Language",
            vec![
                "Rust",
                "Python",
                "C/C++",
                "JavaScript",
                "Dart",
                "Prolog",
                "Zig",
                "PLC Programming",
            ],
            "7B1FA2",
        ),
        (
            "Software Frameworks",
            vec![
                "OpenCV",
                "Yew.rs",
                "Rust Ecosystem",
                "PyTorch",
                "TensorFlow",
                "React.js",
                "Vue.js",
                "node.js",
                "Flutter",
            ],
            "1976D2",
        ),
        (
            "Information Software",
            vec![
                "Typst",
                "Latex",
                "Figma",
                "Microsoft Word/Excel/PowerPoint",
                "Adobe Illustrator",
            ],
            "388E3C",
        ),
        (
            "Engineering Software",
            vec![
                "SOLIDWORKS",
                "Solid Edge",
                "AutoCAD",
                "MATLAB",
                "Linux",
                "TwinCAT",
                "RISC-V Assembly",
            ],
            "EF6C00",
        ),
    ]
    .iter()
    .map(|skill| {
        html! {
            <article class={"card"}>
                <h3>{skill.0}</h3>

                <div class={"skills"}>
                {
                    skill.1.iter().map(|s| html!{
                        <div
                            class={"skill"}
                            style={format!("border-color: #{0}; color: #{0}", skill.2)}
                        >{s}</div>
                    }).collect::<Vec<_>>()
                }
                </div>
            </article>
        }
    })
    .collect::<Vec<_>>();

    html! {
        <section id={"skills"}>
            <h2>
            {"Skills"}
            </h2>
            {skills}
        </section>
    }
}
