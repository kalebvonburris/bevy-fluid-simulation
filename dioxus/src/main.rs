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
                        "Bevy is built with an ", b { "entity-component-system" }, " ((ECS)), which means that data is stored as components; think health bars, movement speed, textures, etc. Components are bundled together with an id, and each of these bundles is called an ", i { "entity" }, ". ", i { "Systems" }, " are functions that take ", i {"queries"}, " and perform operations on them; translations, mutations, collisions, etc. Bundling systems together creates " i { "plugins" }, " and a bundle of plugins is a game. You can also insert ", i { "resources" }, " data that isn't attached to entities in your world; things like terrain, sounds, music, global variables, etc."
                    }

                    br {}

                    h2 {
                        class: "mb-4 text-2xl font-bold leading-tight text-gray-900 lg:mb-6 lg:text-2xl dark:text-white",
                        "Building a Simulation"
                    }

                    p {
                        "My simulation began as a few simple particles, represented as a blue circles on a background. Each had a ", code { "Velocity" }, " , ", code { "Position" }, " , and ", code { "CircleCollider" }, " component that define their absolute velocities, positions, and radius respectively. I used a ", code { "simulate()" }, " function that takes a query to these components; utilizing Bevy's ECS to aquire entities that only had a ", code { "Velocity" }, " , ", code { "Position" }, " , and ", code { "CircleCollider" }, " . I also injected the simulation with a constant: ", code { "Gravity" }, " , which I then applied to each entity's ", code { "Velocity" }, " . Finally, at the end of the ", code { "simulate()" }, " function, I added each particle's ", code { "Velocity" }, " to its ", code { "Position" } " , effectively simulating gravity. This last step would remain constant throughout the development of the simulation."
                    }

                    br{}

                    h2 {
                        class: "mb-4 text-2xl font-bold leading-tight text-gray-900 lg:mb-6 lg:text-2xl dark:text-white",
                        "Defining Collisions and Dispersion"
                    }

                    p {
                        "Two primary aspects would then be needed to create a fluid simulation: collision detection and dispersion. Collision is defined in two categories: inter-particle and inter-boundary."
                    }

                    br {}

                    p {
                        "For inter-particle collisions, we check if the distance between the two particles is less than the sum of their radii. In that case, we also check to see if the particles are travelling towards each other. ", i { "Not having this check resulted in particles \"sticking\" together. We do this by computing the difference in position dotted with the negative difference in velocity. The value will be positive if the particles are pointing towards each other"}, ". If both cases are met, we apply the equation:"
                    }

                    p {
                        "$$  u_1 = v_1 - \\frac{{2m_{{2}}}}{{m_1 + m_2}}\\frac{{\\langle v_1 - v_2, x_1 - x_2 \\rangle}}{{|| x_2 - x_1 ||^2}} (x_1 - x_2) $$",
                        "$$  u_2 = v_2 - \\frac{{2m_{{1}}}}{{m_1 + m_2}}\\frac{{\\langle v_2 - v_1, x_2 - x_2\\rangle}}{{|| x_2 - x_1 ||^2}} (x_2 - x_1) $$"
                    }

                    p {
                        "And then set the velocities of the particles to their respective values here, $u_1$ and $u_2$. These equations follow the conservation of momentum and thus our particles are at least accurate within these interactions."
                    }

                    br {}

                    p {
                        "For inter-boundary collisions, I used the borders of the window to define the boundaries, recieving those details via a resource call: ", code { "Res<&Window>" }, " . Notibly, calls to resources in Bevy are not queries, this is one area where Bevy's ECS can rear its head if you're not paying attention. Using the ", code { "&Window" }, " reference, we can get its width and height with ", code { "window.width()" }, " and ", code { "window.height()" }, " . We can interpret the position of the particles relative to these boundaries with ", code { "window.width() / 2.0" }, " and ", code { "window.height() / 2.0" }, " , any particle exceeding the positive and negative of these boundaries then has its x or y coordinate reset to the boundary and its x or y velocity component reflected as well."
                    }

                    br {}

                    p {
                        "For the dispersion force, we can define the velocity changes to each particle as the distance between the two minus a constant, ", code { "SMOOTHING_RADIUS" }, " . This also has the consequence of allowing us to only search for particles within the ", code { "SMOOTHING_RADIUS" }, " radius."
                    }

                    br {}

                    h2 {
                        class: "mb-4 text-2xl font-bold leading-tight text-gray-900 lg:mb-6 lg:text-2xl dark:text-white",
                        "Optimizations"
                    }

                    p {
                        "That's pretty much all that goes into a fluid simulation; a bunch of math and some rules to define behaviors. However, these rules on their own are not very efficient, with my first few iterations yielding acceptable FPS values of ~60fps with only a hundred particles."
                    }

                    br {}

                    p {
                        "To fix this, I came up with two solutions: ", b { "Multithreading" }, " and ", b { "Chunking" }, ". Both of these combined resulted in similar acceptable framerates of ~60fps with up to 20,000 particles."
                    }

                    br {}

                    p {
                        b { "Multithreading" }, " is the process of splitting up code across multiple threads to do work in parallel. Theoretically, if your machine had 128 cores, it could compute operations in parallel 128x faster than it currently does. This is typically not feasible though as at some point the threads have to communicate with each other, and a \"main\" thread has to collect and process all of the data processed in parallel, resulting in large bottlenecks if your system is not carefully defined. This process is also typically very syntactically difficult to implement in many languages, and so projects like OpenMP were created to turn this problem into a one-line with ", code { "#pragma omp ..." }, " . Luckily Rust has a similar library called ", code { "Rayon" }, " which allows you to parallelize iterators by turning their ", code { ".iter()" }, " calls into ", code { ".par_iter()" }, " calls. This is so useful that Bevy has this built-in to Queries, meaning that Rayon isn't needed for most of code. Implementing this increased the performance on my 8-core AMD machine to ~5,000 particles @ 60fps."
                    }

                    br {}

                    p {
                        b { "Chunking" }, " is the process of splitting up a large amount of data into smaller chunks, which can be accessed in parallel and reduce the number of distance calculations by a significant degree. My implementation chunks the world space and stores each chunk under an ", code { "RwLock" }, " a Rust data structure that allows multiple threads to read a piece of data in parallel, but locks it from reading when a write call is made to allow only a single thread to modify the underlying data. The alternative would be to use a ", code { "HashMap" }, " to store the chunks, but this would force each chunk to be entirely closed off from reading while any other thread is writing it, resulting in poor performance. There almost certainly is a superior approach to this problem, however I found myself short on time while I was implementing it. However, even as it is, the performance on my machine saw a dramatic increase to 20,000 particles @ 60fps."
                    }

                    br {}


                    h2 {
                        class: "mb-4 text-2xl font-bold leading-tight text-gray-900 lg:mb-6 lg:text-2xl dark:text-white",
                        "Final Notes"
                    }

                    p {
                        "As of right now, the simulation is CPU-bound, meaning that all compuations are done entirely on the CPU, although I do make liberal use of Bevy's ", code { "par_iter()" }, " function to parallelize them. Boon here is that your GPU can run thousands if not millions of computations in parallel, resulting in significant performance improvements of probably 10x-100x depending on the graphics card, if not more if the memory is persistent on the GPU. Bevy supports a library called ", b { "wgpu" }, ", a WebGPU API for Rust. This is awesome because WebGPU is a cross-platform, cross-graphics API... graphics API... that allows you to define regular render and compute tasks for the GPU. I look forward to expanding on this project with wgpu in the future, and I've even done some experiementation ", b { a { class: "text-blue-700", href: "https://github.com/kalebvonburris/rust-gpu-testing", "here" } }, " computing fibonacci numbers on the CPU and GPU and comparing your machine's performance."
                    }

                    br {}

                    p {
                        "This article was written in ", b {  a { class: "text-blue-700", href: "https://dioxuslabs.com", "Dioxus" } }, ", a Rust web framework! Yes, that's right! No raw Javascript or Javascript frameworks were used to harm the programmer in the making of this article. You can find the source code ", b { a { class: "text-blue-700", href: "https://github.com/kalebvonburris/bevy-fluid-simulation/blob/main/dioxus/src/main.rs", "here" } }, " and I think that's kinda neat. I was even able to talk to the lovely dev team at the Dioxus discord server about some GitHub pages hosting issues I had and recieved genuinely useful help within a few hours."
                    }
                }
            }
        }
    })
}
