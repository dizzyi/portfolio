use crate::*;

#[function_component]
pub fn Projects() -> Html {
    let i_proj = [
        (
            "Architect, Engineer",
            "Robotics Bars",
            "Aug 2023 - May 2024",
            "mWT8CetyHb8",
            vec![
                r#"Developed a robotic bartending system ("Robo-Mixologist") for a Hong Kong Airport installation, featuring automated cocktail mixing and a touchscreen ordering interface."#,
                r#"Designed and implemented advanced algorithms for precise ingredient measurement, drink mixing, and stylish presentation within a futuristic bar setting."#,
            ],
        ),
        (
            "Architect, Engineer",
            "TSS Process Automation",
            "Aug 2022 - May 2024",
            "SwUt4mq-EXE",
            vec![
                r#"Developed robotic solutions to automate repetitive lab processes like Total Suspended Solids wastewater tests, integrating data capture into business systems."#,
                r#"Streamlined environmental testing workflows by implementing robotics for increased efficiency and reduced manual data entry."#,
            ],
        ),
        (
            "Architect, Engineer",
            "Robotics Barista",
            "",
            "r4o8o6aQE0s",
            vec![
                r#"Automated robotic barista (MyMixx) crafting pour over, French press, and drip coffee via articulated robotic arms."#,
                r#"Execute barista operations for optimal efficiency and consistent quality."#,
            ],
        ),
    ]
        .iter()
        .map(|proj| {
            html! {
                <article class={"card i_proj"}>
                    <h3>
                        {proj.1}
                    </h3>
                    <h4>
                        {proj.0}
                    </h4>
                    <p class="light">
                        {proj.2}
                    </p>
                    {
                        proj.4.iter().map(|l| html!{ <p> {l} </p> }).collect::<Vec<_>>()
                    }
                    <iframe 
                        src={format!("https://www.youtube.com/embed/{}",proj.3)} 
                        title="YouTube video player" 
                        frameborder="0" 
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" 
                        referrerpolicy="strict-origin-when-cross-origin" 
                        allowfullscreen=true>
                    </iframe>
                </article>
            }
        })
        .collect::<Vec<_>>();

    let mut u_proj = [
        (
            "Visual Preprocess for DRL",
            vec![
                "The objective of the project is to develop a artifial solution for industrial warehose bin-picking, using cutting edge technologies like DRL, graph NN and hindsight experience replay",
                "To do that we must transfrom the visual information to a graph which each node contain information of the object seen in the camera",
                "I handle the preprocess using Detectron2's maskRCNN to get information of each object such as the position and segmentation mask.",
                "And then we will pass it through a NN to extract the feature vectors aka nodes.",
            ],
            Some(("Poster", "Poster Modified.pdf")),
            vec![
                "sri1.png",
                "sri2.png",
                "sri3.png",
                "sri32.png",
                "sri4.png",
            ]
        ),
        (
            "Robotics Exoskeleton for load Transportation",
            vec![
               "Final Year Project",

               "This project objective is to develop an light-weighted Exoskeleton robot for transportation with both active and passive technique",
               "To achieve that, in the hip joint of the exoskeleton, there are a third beam attached instead of just back and leg beam",

               "This cause a problem, since the third beam required different behaivor in different mode of the motion, i.e. walking and lifting mode.",
               "I proposed a mechanism design with planetary gear, which can couple the three beam and achieve desired result.",

                           ],
            Some(("Report", "Final Year Project Final Report.pdf")),
            vec![
                "fyp1.png",
                "fyp2.png",
                "fyp3.png",
                "fyp4.png",
                "fyp5.png",
            ]
        ),(
            "Code Testing Platform",
            vec![
                "Build using C# and WFP. This project used concept of Object Oriented Programming and Design Patter.",
                "This App allow user to upload their code to answer certain problem, the app will compile those code and check if the student answer correctly",
                "The admin can view the passrate of the problem, create new problem and edit problems.",
            ],
            None,
            vec![
                "swe1.png",
                "swe2.png",
                "swe3.png",
                "swe4.png",
                "swe5.png",
            ]
        )
    ]
    .iter()
    .map(|proj|html!{
        <article class="card u_proj">
            <div class="u_proj_left">
                <h3>
                    {proj.0}
                </h3>
                {
                    proj.1.iter().map(|i|
                        html!{
                            <p> {i} </p>
                        }
                    ).collect::<Vec<_>>()
                }
                if let Some(b) = proj.2 {
                    <a href={format!("./media/{}", b.1)}> {b.0} </a>
                }
            </div>
            <div class="u_proj_right">
                {
                    proj.3.iter().map(|i|
                        html!{
                            <img src={format!("./media/{}", i)} />
                        }
                    ).collect::<Vec<_>>()
                }
            </div>
        </article>
    })
    .collect::<Vec<_>>();

    u_proj.insert(0,html! {
        <article class="card u_proj" id={"leg"}>
            <div class="u_proj_left">
                <h3>
                    {"Bionic Robot Leg"}
                </h3>
                <p>
                    {"Inspired by 2019 Robocon's mission, this project attempt to develop a leg for a 4 leged robot."}
                </p>
                <p>
                    {"We use SOLIDWORK to design the 3D model, then manufacture using acrylic sheet and M3 screw."}
                </p>
                <p>
                    {"The project use Arduino Uno as microcontroller, which use CAN bus communication to command the motor controller."}
                </p>
                <a href="./media/MAEG2050-Presentation.pdf">
                    {"Report"}
                </a>
                <canvas id={"robotlegcanvas"}/>
            </div>
            <div class="u_proj_right">
                {
                    (0..4).map(|i|
                        html!{
                            <video loop=true muted=true autoplay=true preload={"none"}>
                                <source src={format!("./media/MAEG2050-{}.mp4",i+1)}/>
                            </video>
                        }
                    ).collect::<Vec<_>>()
                }
            </div>
        </article>
    });

    u_proj.insert(1, html! {
        <article class="card u_proj" id={"arm"}>
            <div class="u_proj_left">
                <h3>
                    {"Robotics Arm Control"}
                </h3>
                <p>
                    {"The project design for medical and pharmaceutical industry, will have a HMI for user to input needed drug, and the robot arm will deliver to the destination"}
                </p>
                <p>
                    {"This project mainly apply the knowledge of forward kinematic, inverse kinematic and how to avoid singularity."}
                </p>
                <video loop=true muted=true autoplay=true preload={"none"}>
                    <source src={"./media/MAEG3060.mp4"}/>
                </video>
            </div>
        </article>
    });

    html! {
        <section id={"projects"}>
            <h2>
                {"Projects"}
            </h2>

            <h2 class={"sub_h2"}>
                {"Industry Projects"}
            </h2>
            <div id="proj_i">
                { i_proj }
            </div>


            <h2 class={"sub_h2"}>
                {"University Projects"}
            </h2>
            <div id="proj_u">
                { u_proj }
            </div>


        </section>
    }
}
