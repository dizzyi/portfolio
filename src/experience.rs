use crate::*;

pub struct Exp {
    pub title: String,
    pub org: String,
    pub date: String,
    pub res: Vec<String>,
}

impl Exp {
    pub fn new(
        title: impl Into<String>,
        org: impl Into<String>,
        date: impl Into<String>,
        res: impl IntoIterator<Item = impl Into<String>>,
    ) -> Exp {
        Exp {
            title: title.into(),
            org: org.into(),
            date: date.into(),
            res: res.into_iter().map(Into::into).collect(),
        }
    }
}

#[function_component]
pub fn Experience() -> Html {
    let res = vec![
        Exp::new(
            "Mechanical Engineer",
            "Inovo Robotics",
            "Aug 2022 - present",
            vec![
                "Design and integrate industrial automation solutions, using robot arms, various sensors, and actuators.",
                "Conduct experiments and interpret data for quality control, with scientific computing skills."
            ],
        ),
        Exp::new(
            "Research Intern",
            "CUHK Engineeringg Faculty & HKCLR",
            "Apr 2021 - Aug 2021",
            vec![
                "Read academic papers on cutting edge Machine Learning Methods e.g., Deep Reinforcement Learning and graph Neural Networks.",
                "Aided research team in construction solution for industrial palletizing, using PyTorch."
            ],
        ),
        Exp::new(
            "Treasurer of Student Publication Committee",
            "Shaw College Student Union",
            "Mar 2019 - Feb 2020",
            vec![
                "Managed committee's finance and budge.",
                "Coordinated with other committees and printing company",
                "Wrote and designed issues of publication (Shaw Beat).",
            ],
        ),
        Exp::new(
            "Lego Robotics Teacher",
            "Carmel Alison Lam Foundation Secondary School",
            "Sep 2018 - Aug 2021",
            vec![
                "Guide 15-20 secondary students to build and program Lego robots.",
                "Prepare students for public robot competition.",
            ],
        ),
    ];

    let res: Vec<_> = res
        .into_iter()
        .map(|e| {
            let res: Vec<_> = e
                .res
                .iter()
                .map(|r| {
                    html! {
                        <li>
                            {r}
                        </li>
                    }
                })
                .collect();

            html! {
                <article class={"card"}>
                    <div>
                        <div>
                            <h3>
                                {e.title}
                            </h3>
                            <h4>
                                {e.org}
                            </h4>
                        </div>
                        <div>
                            <p>
                                {e.date}
                            </p>
                        </div>
                    </div>
                    <ul>
                        {res}
                    </ul>
                </article>
            }
        })
        .collect();

    html! {
        <section id={"experience"}>
            <h2>
                {"Experience"}
            </h2>
            <div>
                {res}
            </div>
        </section>
    }
}
