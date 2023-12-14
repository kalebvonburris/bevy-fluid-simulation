// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the web app
    dioxus_web::launch(app);
}

// create a component that renders a div with the text "Hello, world!"
fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "pt-8 pb-16 lg:pt-16 lg:pb-24 bg-white dark:bg-gray-900 antialiased text-gray-500 dark:text-gray-400 h-screen",
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
                        "Bevy is build with an ", b { "entity-component-system" }, " (ECS), which means that data is stored as components; think health bars, movement speed, textures, etc. Components are bundled together with an id, and each of these bundles is called an ", i { "entity" }, ". ", i { "Systems" }, " are functions that take ", i {"queries"}, " and perform operations on them; translations, mutations, collisions, etc. Bundling systems together creates " i { "plugins" }, " and a bundle of plugins is a game. You can also insert ", i { "resources" }, " data that isn't attached to entities in your world; things like terrain, sounds, music, global variables, etc."
                    }

                    br {}

                    h2 {
                        class: "mb-4 text-3xl font-bold leading-tight text-gray-900 lg:mb-6 lg:text-3xl dark:text-white",
                        "Building a Simulation"
                    }

                    p {
                        ""
                    }
                }
            }
        }
    })
}
