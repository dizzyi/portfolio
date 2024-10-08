use crate::*;

#[function_component]
pub fn Education() -> Html {
    let highlight = [
        (
            "Mechanical",
            vec![
                "Material Mechanics",
                "Manufacturing Technologies",
                "Computer Aided Manufacturing",
            ],
        ),
        (
            "Electronics",
            vec![
                "Electronic Circuit Analysis",
                "Microelectronic Application",
                "Computer Organization",
            ],
        ),
        (
            "Control",
            vec![
                "Modern Control Theory",
                "Advance Robotics",
                "Deep Learning Control",
                "Automation Control",
            ],
        ),
        (
            "Software",
            vec![
                "Artifical Intelligence",
                "Software Engineering",
                "Web Development",
            ],
        ),
    ];

    let highlight = highlight
        .into_iter()
        .enumerate()
        .map(|(i, (t, v))| {
            html! {
                <div class={format!("card card_{}", i)}>
                <h4> {t} </h4>
                {
                    v.into_iter()
                    .map(|i| html!{
                        <h5>{i}</h5>
                    }).collect::<Vec<_>>()
                }
                </div>
            }
        })
        .collect::<Vec<_>>();

    html! {
        <section id={"education"}>
            <h2>
                {"Education"}
            </h2>
            <div id={"edu_top"}>
                <div id={"edu_top_left"}>
                    <h3>
                        {"The Chinese University of Hong Kong"}
                    </h3>
                    <h4>
                        {"Bachlor of Engineering in "}
                        <span class={"highlight"}>
                            {"Mechanical and Automation Engineering"}
                        </span>
                    </h4>
                    <h5>
                        {"Honors: Dean's List 2020-2021"}
                    </h5>
                    <h5>
                        {"Cumlative GPA: 3.371/4.000"}
                    </h5>
                </div>
                <div id={"edu_top_right"}>
                    <div id="edu_pic"/>
                </div>
            </div>
            <div id={"edu_bottom"}>
                <h4>
                    {"Highlight"}
                </h4>
                <div id="edu_hl">
                    {highlight}
                </div>
            </div>
        </section>
    }
}
