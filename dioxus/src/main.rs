// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

// create a component that renders a div with the text "Hello, world!"
fn app(cx: Scope) -> Element {
    let code = "MathJax = {
        tex: {
            inlineMath: [['$', '$'], ['\\(', '\\)']]
        }
    };".to_string();
    let call_js_fn = use_eval(&cx);
    call_js_fn(&code).unwrap();

    cx.render(rsx! {
        head {
            script {
                src: "https://polyfill.io/v3/polyfill.min.js?features=es6"
            }
            script {
                "id": "MathJax-script",
                src: "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"
            }
        }
        main {
            class: "pt-8 pb-16 lg:pt-16 lg:pb-24 bg-white dark:bg-gray-900 antialiased text-gray-500 dark:text-gray-400",
            
            div {
                article {
                    class: "mx-auto w-full max-w-4xl format format-sm sm:format-base lg:format-lg format-blue dark:format-invert",
                    header {
                        class: "mb-4 lg:mb-6 not-format",
                        address {
                            class: "flex items-center mb-6 not-italic",
                            div {
                                class: "inline-flex items-center mr-3 text-sm text-gray-900 dark:text-white",
                                div {
                                    a {
                                        class: "text-xl font-bold text-gray-900 dark:text-white",
                                        rel: "author",
                                        "Kaleb Burris"
                                    }
                                    p {
                                        class: "text-base text-gray-500 dark:text-gray-400",
                                        "Computer Science Major at the University of Alaska Fairbanks & Student Intern at the Alaska Satellite Facility"
                                    }
                                    p {
                                        class: "text-base text-gray-500 dark:text-gray-400",
                                        time {
                                            datetime: "2023-12-14",
                                            class: "text-base text-gray-500 dark:text-gray-400",
                                            "December 14, 2023"
                                        }
                                    }
                                }
                            }
                        }
                        h1 {
                            class: "mb-4 text-3xl font-extrabold leading-tight text-gray-900 lg:mb-6 lg:text-4xl dark:text-white",
                            "Bevy Fluid Simulation"
                        }
                    }
                    
                    p {
                        class: "lead",
                        "For my CS 301 ", b { "Introduction to assembly"}, " class we were given the task of coming up with and executing a project relating to assembly, optimizations, multithreading, and/or GPU compute."
                    }

                    br {}

                    p {
                        "For the project, I'd recently seen ", b { a { class: "text-blue-700", href: "https://youtu.be/rSKMYc1CQHE?si=NXZc9vEJqeMCfxt4", "Sebatian Lague's video on making a fluid simulation in Unity" } }, " and was immediately inspired to replicate at least some of it. As I already had a lot of experience with the programming language ", b { a { class: "text-blue-700", href: "https://www.rust-lang.org/", "Rust" } }, " I decided to try my version in Rust, and thus would need a Rust-based game engine. The most most popular option for Rust game engines is ", b { a { class: "text-blue-700", href: "https://bevyengine.org/", "Bevy" } }, ", an ECS-driven engine that promised performance in a lightweight and portable package."
                    }

                    br {}

                    h2 {
                        class: "mb-4 text-3xl font-bold leading-tight text-gray-900 lg:mb-6 lg:text-3xl dark:text-white",
                        "Bevy and the ECS"
                    }

                    p {
                        "Bevy is built with an ", b { "entity-component-system" }, " (ECS), which means that data is stored as components; think health bars, movement speed, textures, etc. Components are bundled together with an id, and each of these bundles is called an ", i { "entity" }, ". ", i { "Systems" }, " are functions that take ", i {"queries"}, " and perform operations on them; translations, mutations, collisions, etc. Bundling systems together creates " i { "plugins" }, " and a bundle of plugins is a game. You can also insert ", i { "resources" }, " data that isn't attached to entities in your world; things like terrain, sounds, music, global variables, etc."
                    }

                    br {}

                    h2 {
                        class: "mb-4 text-3xl font-bold leading-tight text-gray-900 lg:mb-6 lg:text-3xl dark:text-white",
                        "Building a Simulation"
                    }

                    p {
                        "My simulation began as a few simple particles, represented as a blue circles on a background. Each had a ", code { "Velocity" }, " , ", code { "Position" }, " , and ", code { "CircleCollider" }, " component that define their absolute velocities, positions, and radius respectively. I used a ", code { "simulate()" }, " function that takes a query to these components; utilizing Bevy's ECS to aquire entities that only had a ", code { "Velocity" }, " , ", code { "Position" }, " , and ", code { "CircleCollider" }, " . I also injected the simulation with a constant: ", code { "Gravity" }, " , which I then applied to each entity's ", code { "Velocity" }, " . Finally, at the end of the ", code { "simulate()" }, " function, I added each particle's ", code { "Velocity" }, " to its ", code { "Position" } " , effectively simulating gravity. This last step would remain constant throughout the development of the simulation."
                    }

                    br{}

                    h2 {
                        class: "mb-4 text-3xl font-bold leading-tight text-gray-900 lg:mb-6 lg:text-3xl dark:text-white",
                        "Defining Collisions and Dispersion"
                    }

                    p {
                        "Two primary aspects would then be needed to create a fluid simulation: collision detection and dispersion. Collision is defined in two categories: inter-particle and inter-boundary."
                    }

                    br {}

                    p {
                        "For inter-particle collisions, we check if the distance between the two particles is less than the sum of their radii. In that case, we also check to see if the particles are travelling towards each other. ", i { "Not having this check resulted in particles \"sticking\" together"}, ". If both cases are met, we apply the equation:"
                    }

                    p {
                        "$$  u_1 = v_1 - \\frac{{2m_{{2}}}}{{m_1 + m_2}}\\frac{{\\langle v_1 - v_2, x_1 - x_2 \\rangle}}{{|| x_2 - x_1 ||^2}} (x_1 - x_2) $$",
                        "$$  u_2 = v_2 - \\frac{{2m_{{1}}}}{{m_1 + m_2}}\\frac{{\\langle v_2 - v_1, x_2 - x_2\\rangle}}{{|| x_2 - x_1 ||^2}} (x_2 - x_1) $$"
                    }

                    br {}

                    p {
                        "For inter-boundary collisions, I used the borders of the window to define the boundaries, recieving those details via a resource call: ", code { "Res<&Window>" }, " . Notibly, calls to resources in Bevy and not queries. "
                    }

                    br {}

                    p {
                        "For the dispersion force, we can define the velocity changes to each particle as the distance between the two minus a constant, ", code { "SMOOTHING_RADIUS" }, " . This also has the consequence of allowing us to only search for particles with the ", code { "SMOOTHING_RADIUS" }, " radius."
                    }

                    p {
                        "As of right now, the simulation is CPU-bound, meaning that all compuations are done entirely on the CPU, although I do make liberal use of Bevy's ", code { "par_iter()" }, " function to parallelize them. The issue with this is that your GPU can run thousands if not millions of computations in parallel, "
                    }
                }
            }
        }
    })
}
