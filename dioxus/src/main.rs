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
            class: "pt-8 pb-16 lg:pt-16 lg:pb-24 bg-white dark:bg-gray-900 antialiased text-gray-500 dark:text-gray-400 my-0 top-0 bottom-0",
            div {
                article {
                    class: "mx-auto w-full max-w-2xl format format-sm sm:format-base lg:format-lg format-blue dark:format-invert",
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
                                        "Computer Science Major at the University of Alaska Fairbanks"
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
                        "For the project, I elected to go with a fluid simulation and starting looking for solutions of how I would implement this. As I already had a lot of experience with the programming language ", b { "Rust" }, " I decided a game engine in ", b { "Rust" }, " would be the best choice."
                    }

                    br {}
                }
            }
        }
    })
}
