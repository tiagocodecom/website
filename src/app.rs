use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta
                    name="description"
                    content="Leptos is a cutting-edge Rust web framework designed for building fast, reliable, web applications."
                />
                <link rel="stylesheet" id="leptos" href="/pkg/tiagocode_website.css"/>
                <link rel="stylesheet" href="/assets/plugins/bootstrap-icons/bootstrap-icons.min.css"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body class="overflow-x-hidden bg-bodyBg font-opensans dark:bg-black">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Title text="Welcome to Leptos"/>
        <Router>
            <Routes fallback=|| "Not found.">
                <Route path=path!("") view=HomePage/>
            </Routes>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="toggle-menu z-20 toggle-menu fixed top-0 right-0 translate-x-3 w-96 h-full bg-black dark:bg-boxDark dark:shadow-darkBox px-10 py-12 invisible opacity-0 transition-all ease-out duration-[160ms]">
            <span class="block font-mono font-normal uppercase text-sm tracking-[0.5px] text-white mb-2">Phone:</span>
            <h4 class="font-poppins font-medium text-2xl text-white">+(976) 1234 4444</h4>
            <div class="mt-6">
                <span class="block font-mono font-normal uppercase text-sm tracking-[0.5px] text-white mb-2">Email:</span>
                <h4 class="font-poppins font-medium text-2xl text-white">flatheme@gmail.com</h4>
            </div>
            <ul class="space-x-2 mt-4">
                <li class="list-none inline-block">
                    <a class="inline-flex justify-center items-center bg-white/15 w-10 h-10 rounded-full text-white transition ease-out duration-[120ms] hover:bg-white/20" href="#">
                        <i class="bi bi-facebook"></i>
                    </a>
                </li>
                <li class="list-none inline-block">
                    <a class="inline-flex justify-center items-center bg-white/15 w-10 h-10 rounded-full text-white transition ease-out duration-[120ms] hover:bg-white/20" href="#">
                        <i class="bi bi-twitter-x"></i>
                    </a>
                </li>
                <li class="list-none inline-block">
                    <a class="inline-flex justify-center items-center bg-white/15 w-10 h-10 rounded-full text-white transition ease-out duration-[120ms] hover:bg-white/20" href="#">
                        <i class="bi bi-instagram"></i>
                    </a>
                </li>
            </ul>
            <ul class="mt-10 space-y-2">
                <li
                    class="relative pl-3 before:content-[''] before:absolute before:top-1/2 before:left-0 before:-translate-y-1/2 before:bg-white before:opacity-70 before:w-1 before:h-1 before:rounded-full before:transition before:ease-linear before:duration-100 hover:before:opacity-100"
                >
                    <a class="font-mono font-medium uppercase text-sm tracking-[0.5px] text-white hover:underline" href="index-dark.html">Dark version</a>
                </li>
                <li
                    class="relative pl-3 before:content-[''] before:absolute before:top-1/2 before:left-0 before:-translate-y-1/2 before:bg-white before:opacity-70 before:w-1 before:h-1 before:rounded-full before:transition before:ease-linear before:duration-100 hover:before:opacity-100"
                >
                    <a class="font-mono font-medium uppercase text-sm tracking-[0.5px] text-white hover:underline" href="#">Buy Gray</a>
                </li>
            </ul>
            <div class="absolute bottom-12 left-10 right-10">
                <p class="text-white/70">&copy; 2024 FlaTheme.</p>
            </div>
            <button class="menu-close absolute top-4 right-4 inline-flex justify-center items-center bg-white/15 w-10 h-10 rounded-full text-white text-xl transition ease-out duration-150 hover:bg-white/20">
                <i class="bi bi-x"></i>
            </button>
        </div>

        <div class="container max-w-[1320px] mx-auto px-5 xl:px-3">
            <div class="lg:flex lg:justify-between">
                <div class="flex h-[50px] items-center space-x-6 lg:order-2 justify-end">
                    <ul class="space-x-3.5 font-mono font-medium uppercase text-sm tracking-[0.5px]">
                        <li class="list-none inline-block"><a class="hover:underline dark:text-white" href="#">FB</a></li>
                        <li class="list-none inline-block"><a class="hover:underline dark:text-white" href="#">TW</a></li>
                        <li class="list-none inline-block"><a class="hover:underline dark:text-white" href="#">IG</a></li>
                        <li class="list-none inline-block"><a class="hover:underline dark:text-white" href="#">IN</a></li>
                    </ul>
                    <button class="menu-btn group relative w-[50px] h-[50px] bg-black dark:bg-boxDark rounded-b-lg">
                        <span class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 bg-white w-1 h-1 rounded-full transition-all ease-linear duration-100 delay-100 group-hover:scale-[3]"></span>
                        <span
                            class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-1 h-1 before:content-[''] before:absolute before:top-0 before:-left-[10px] before:bg-white before:w-1 before:h-1 before:rounded-full before:transition-all before:ease-linear before:duration-100 after:content-[''] after:absolute after:top-0 after:-right-[10px] after:bg-white after:w-1 after:h-1 after:rounded-full after:transition-all after:ease-linear after:duration-100 group-hover:before:left-0 group-hover:before:opacity-0 group-hover:after:right-0 group-hover:after:opacity-0"
                        ></span>
                    </button>
                </div>
                <div class="py-7 lg:order-1">
                    <h1 class="text-5xl xl:text-7xl font-poppins font-semibold dark:text-white">Santiago <span class="stroke-text">Marulanda</span></h1>
                </div>
            </div>

            <div class="space-y-6 lg:flex lg:space-x-8 lg:space-y-0 xl:space-x-12">
                <div class="z-10 sticky top-2 lg:top-6 lg:h-fit lg:w-1/4 bg-black/90 dark:bg-boxDark backdrop-blur-[5px] rounded-lg px-4 py-3 lg:bg-black lg:px-8 lg:py-5 xl:px-10 xl:py-7 lg:backdrop-blur-none">
                    <ul class="font-mono font-normal uppercase text-sm tracking-wider text-center lg:text-left space-x-4 lg:space-x-0">
                        <li class="list-none inline-block lg:block">
                            <a
                                class="section-link active group inline-flex justify-center items-center lg:block lg:justify-normal relative w-9 h-9 border border-transparent border-dashed rounded-full lg:w-auto lg:h-auto lg:border-none lg:rounded-none text-white/70 py-3 transition ease-linear duration-100 hover:text-white"
                                href="#about"
                            >
                                <span class="hidden lg:inline-block">About Me</span><span class="lg:hidden">A</span>
                                <span
                                    class="nav-circle hidden lg:inline-block absolute top-1/2 right-0 -translate-y-1/2 w-[5px] h-[5px] before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-white before:w-[5px] before:h-[5px] before:rounded-full before:opacity-70 before:transition-all before:ease-out before:duration-200 group-hover:before:opacity-100"
                                ></span>
                            </a>
                        </li>
                        <li class="list-none inline-block lg:block">
                            <a
                                class="section-link group inline-flex justify-center items-center lg:block lg:justify-normal relative w-9 h-9 border border-transparent border-dashed rounded-full lg:w-auto lg:h-auto lg:border-none lg:rounded-none text-white/70 py-3 transition ease-linear duration-100 hover:text-white"
                                href="#portfolio"
                            >
                                <span class="hidden lg:inline-block">Portfolio</span><span class="lg:hidden">P</span>
                                <span
                                    class="nav-circle hidden lg:inline-block absolute top-1/2 right-0 -translate-y-1/2 w-[5px] h-[5px] before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-white before:w-[5px] before:h-[5px] before:rounded-full before:opacity-70 before:transition-all before:ease-out before:duration-200 group-hover:before:opacity-100"
                                ></span>
                            </a>
                        </li>
                        <li class="list-none inline-block lg:block">
                            <a
                                class="section-link group inline-flex justify-center items-center lg:block lg:justify-normal relative w-9 h-9 border border-transparent border-dashed rounded-full lg:w-auto lg:h-auto lg:border-none lg:rounded-none text-white/70 py-3 transition ease-linear duration-100 hover:text-white"
                                href="#services"
                            >
                                <span class="hidden lg:inline-block">Services</span><span class="lg:hidden">S</span>
                                <span
                                    class="nav-circle hidden lg:inline-block absolute top-1/2 right-0 -translate-y-1/2 w-[5px] h-[5px] before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-white before:w-[5px] before:h-[5px] before:rounded-full before:opacity-70 before:transition-all before:ease-out before:duration-200 group-hover:before:opacity-100"
                                ></span>
                            </a>
                        </li>
                        <li class="list-none inline-block lg:block">
                            <a
                                class="section-link group inline-flex justify-center items-center lg:block lg:justify-normal relative w-9 h-9 border border-transparent border-dashed rounded-full lg:w-auto lg:h-auto lg:border-none lg:rounded-none text-white/70 py-3 transition ease-linear duration-100 hover:text-white"
                                href="#testimonial"
                            >
                                <span class="hidden lg:inline-block">Testimonial</span><span class="lg:hidden">T</span>
                                <span
                                    class="nav-circle hidden lg:inline-block absolute top-1/2 right-0 -translate-y-1/2 w-[5px] h-[5px] before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-white before:w-[5px] before:h-[5px] before:rounded-full before:opacity-70 before:transition-all before:ease-out before:duration-200 group-hover:before:opacity-100"
                                ></span>
                            </a>
                        </li>
                        <li class="list-none inline-block lg:block">
                            <a
                                class="section-link group inline-flex justify-center items-center lg:block lg:justify-normal relative w-9 h-9 border border-transparent border-dashed rounded-full lg:w-auto lg:h-auto lg:border-none lg:rounded-none text-white/70 py-3 transition ease-linear duration-100 hover:text-white"
                                href="#resume"
                            >
                                <span class="hidden lg:inline-block">Resume</span><span class="lg:hidden">R</span>
                                <span
                                    class="nav-circle hidden lg:inline-block absolute top-1/2 right-0 -translate-y-1/2 w-[5px] h-[5px] before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-white before:w-[5px] before:h-[5px] before:rounded-full before:opacity-70 before:transition-all before:ease-out before:duration-200 group-hover:before:opacity-100"
                                ></span>
                            </a>
                        </li>
                        <li class="list-none inline-block lg:block">
                            <a
                                class="section-link group inline-flex justify-center items-center lg:block lg:justify-normal relative w-9 h-9 border border-transparent border-dashed rounded-full lg:w-auto lg:h-auto lg:border-none lg:rounded-none text-white/70 py-3 transition ease-linear duration-100 hover:text-white"
                                href="#blog"
                            >
                                <span class="hidden lg:inline-block">Blog</span><span class="lg:hidden">B</span>
                                <span
                                    class="nav-circle hidden lg:inline-block absolute top-1/2 right-0 -translate-y-1/2 w-[5px] h-[5px] before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-white before:w-[5px] before:h-[5px] before:rounded-full before:opacity-70 before:transition-all before:ease-out before:duration-200 group-hover:before:opacity-100"
                                ></span>
                            </a>
                        </li>
                        <li class="list-none inline-block lg:block">
                            <a
                                class="section-link group inline-flex justify-center items-center lg:block lg:justify-normal relative w-9 h-9 border border-transparent border-dashed rounded-full lg:w-auto lg:h-auto lg:border-none lg:rounded-none text-white/70 py-3 transition ease-linear duration-100 hover:text-white"
                                href="#contact"
                            >
                                <span class="hidden lg:inline-block">Contact</span><span class="lg:hidden">C</span>
                                <span
                                    class="nav-circle hidden lg:inline-block absolute top-1/2 right-0 -translate-y-1/2 w-[5px] h-[5px] before:content-[''] before:absolute before:top-1/2 before:left-1/2 before:-translate-x-1/2 before:-translate-y-1/2 before:bg-white before:w-[5px] before:h-[5px] before:rounded-full before:opacity-70 before:transition-all before:ease-out before:duration-200 group-hover:before:opacity-100"
                                ></span>
                            </a>
                        </li>
                    </ul>
                </div>

                <div class="lg:w-3/4 space-y-6 pb-12">
                    <div id="about" class="section bg-white dark:bg-boxDark rounded-lg px-6 py-8 md:px-8 md:py-10 lg:p-12 shadow-sectionBoxShadow hover:shadow-sectionBoxShadowHover transition ease-out duration-[160ms]">
                        <div class="lg:flex lg:space-x-10">
                            <div class="relative h-fit mb-6">
                                <img class="min-w-52 min-h-52 max-w-64 max-h-64 rounded-full" src="assets/images/hero-avatar.jpg" alt="" />
                                <div class="absolute bottom-4 left-2 bg-black/30 dark:bg-black/60 px-4 py-2 rounded-full shadow-avatarText backdrop-blur-[5px] text-white font-mono font-normal uppercase text-sm tracking-wider">
                                    <span class="typer" id="typer1" data-words="Hi There!, I'm Christina" data-colors="white" data-delay="50" data-deleteDelay="1500"></span><span class="cursor" data-owner="typer1"></span>
                                </div>
                            </div>
                            <div>
                                <h6
                                    class="font-mono font-medium uppercase text-sm tracking-wider relative pt-4 mb-5 dark:text-white before:content-['//'] before:pr-2 after:content-[attr(data-backdrop-text)] after:absolute after:top-0 after:left-0 after:font-poppins after:font-bold after:uppercase after:text-4xl after:opacity-15"
                                    data-backdrop-text="About Me"
                                >
                                    About Me
                                </h6>
                                <h2 class="text-4xl font-poppins font-semibold mb-2 dark:text-white">
                                    UI & UX Designer. <br />
                                    Photographer
                                </h2>
                                <ul class="space-y-3 mb-4">
                                    <li
                                        class="list-none inline-block px-4 py-2 me-2 rounded-full border border-black/20 dark:border-white/30 border-dashed text-pColor hover:text-black dark:text-white/70 dark:hover:text-white transition ease-linear duration-100"
                                    >
                                        <i class="bi bi-camera pe-1"></i>
                                        Photoshoot
                                        <div class="inline-block font-mono text-sm">"("<span class="counter">94</span>%")"</div>
                                    </li>
                                    <li
                                        class="list-none inline-block px-4 py-2 me-2 rounded-full border border-black/20 dark:border-white/30 border-dashed text-pColor hover:text-black dark:text-white/70 dark:hover:text-white transition ease-linear duration-100"
                                    >
                                        <i class="bi bi-code-slash pe-1"></i>
                                        Tailwind
                                        <div class="inline-block font-mono text-sm">"("<span class="counter">98</span>%")"</div>
                                    </li>
                                    <li
                                        class="list-none inline-block px-4 py-2 me-2 rounded-full border border-black/20 dark:border-white/30 border-dashed text-pColor hover:text-black dark:text-white/70 dark:hover:text-white transition ease-linear duration-100"
                                    >
                                        <i class="bi bi-search pe-1"></i>
                                        SEO
                                        <div class="inline-block font-mono text-sm">"("<span class="counter">85</span>%")"</div>
                                    </li>
                                </ul>
                                <p class="text-pColor dark:text-white/70">Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua</p>
                            </div>
                        </div>
                        <div class="grid grid-cols-1 lg:grid-cols-3 gap-5 mt-8">
                            <div class="flex items-center">
                                <div class="pe-2">
                                    <div class="font-mono font-semibold text-6xl stroke-text"><span class="counter">12</span></div>
                                </div>
                                <div class="dark:text-white">
                                    <span class="block text-2xl font-normal mb-1">+</span>
                                    <p class="font-mono font-medium text-sm uppercase tracking-[0.5px]">Years of Experience</p>
                                </div>
                            </div>
                            <div class="flex items-center">
                                <div class="pe-2">
                                    <div class="font-mono font-semibold text-6xl stroke-text"><span class="counter">20</span></div>
                                </div>
                                <div class="dark:text-white">
                                    <span class="block text-2xl font-normal mb-1">k</span>
                                    <p class="font-mono font-medium text-sm uppercase tracking-[0.5px]">Hours of Working</p>
                                </div>
                            </div>
                            <div class="flex items-center">
                                <div class="pe-2">
                                    <div class="font-mono font-semibold text-6xl stroke-text"><span class="counter">90</span></div>
                                </div>
                                <div class="dark:text-white">
                                    <span class="block text-2xl font-normal mb-1">+</span>
                                    <p class="font-mono font-medium text-sm uppercase tracking-[0.5px]">Projects Done</p>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div id="portfolio" class="section bg-white dark:bg-boxDark rounded-lg px-6 py-8 md:px-8 md:py-10 lg:p-12 shadow-sectionBoxShadow hover:shadow-sectionBoxShadowHover transition ease-out duration-[160ms]">
                        <div class="md:w-4/5 lg:w-3/4">
                            <h6 class="font-mono font-medium uppercase text-sm tracking-wider relative pt-4 mb-5 dark:text-white before:content-['//'] before:pr-2 after:content-[attr(data-backdrop-text)] after:absolute after:top-0 after:left-0 after:font-poppins after:font-bold after:uppercase after:text-4xl after:opacity-15" data-backdrop-text="Portfolio">Portfolio</h6>
                            <h2 class="text-3xl lg:text-4xl font-poppins font-semibold mb-3 lg:mb-4 dark:text-white">My Latest Works</h2>
                            <p class="text-pColor dark:text-white/70">Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore</p>
                        </div>
                        <div class="mt-6 lg:mt-12">
                            <ul class="filter space-x-2">
                                <li class="list-none inline-block font-mono text-sm px-4 py-2 border border-black border-dashed rounded-full hover:bg-black hover:text-white transition ease-linear duration-100 cursor-pointer dark:text-white dark:border-white dark:hover:bg-white dark:hover:text-black mixitup-control-active" data-filter="all">Show All</li>
                                <li class="list-none inline-block font-mono text-sm px-4 py-2 border border-black border-dashed rounded-full hover:bg-black hover:text-white transition ease-linear duration-100 cursor-pointer dark:text-white dark:border-white dark:hover:bg-white dark:hover:text-black" data-filter=".category-1">First</li>
                                <li class="list-none inline-block font-mono text-sm px-4 py-2 border border-black border-dashed rounded-full hover:bg-black hover:text-white transition ease-linear duration-100 cursor-pointer dark:text-white dark:border-white dark:hover:bg-white dark:hover:text-black" data-filter=".category-2">Second</li>
                            </ul>
                            <div class="portfolio-grid grid grid-cols-1 sm:grid-cols-2 gap-6 mt-6">
                                <div class="portfolio-item category-1">
                                    <div class="relative overflow-hidden group rounded-lg after:content-[''] after:absolute after:top-0 after:left-0 after:w-full after:h-full after:bg-gradient-to-t after:from-black/30 after:to-transparent after:opacity-0 after:transition after:ease-out after:duration-[160ms] hover:after:opacity-100 category-1">
                                        <img class="transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.4px]" src="assets/images/portfolio-img.png" alt="" />
                                        <div class="z-[1] absolute top-4 right-4 bg-black/20 px-4 py-2 rounded-full backdrop-blur-[5px] text-white font-mono font-normal text-sm uppercase tracking-[0.5px]">
                                            Category
                                        </div>
                                        <div class="z-[1] absolute bottom-0 left-0 w-full px-7 pb-6 invisible opacity-0 translate-y-2 group-hover:translate-y-0 group-hover:visible group-hover:opacity-100 group-hover:mb-0 transition ease-out duration-[160ms]">
                                            <a class="font-poppins font-semibold text-3xl lg:text-4xl tracking-[0.5px] portfolio-stroke-text transition-all ease-linear duration-100" href="portfolio-single.html">Project Title</a>
                                        </div>
                                    </div>
                                </div>
                                <div class="portfolio-item category-2">
                                    <div class="relative overflow-hidden group rounded-lg after:content-[''] after:absolute after:top-0 after:left-0 after:w-full after:h-full after:bg-gradient-to-t after:from-black/30 after:to-transparent after:opacity-0 after:transition after:ease-out after:duration-[160ms] hover:after:opacity-100 category-1">
                                        <img class="transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.4px]" src="assets/images/portfolio-img.png" alt="" />
                                        <div class="z-[1] absolute top-4 right-4 bg-black/20 px-4 py-2 rounded-full backdrop-blur-[5px] text-white font-mono font-normal text-sm uppercase tracking-[0.5px]">
                                            Category
                                        </div>
                                        <div class="z-[1] absolute bottom-0 left-0 w-full px-7 pb-6 invisible opacity-0 translate-y-2 group-hover:translate-y-0 group-hover:visible group-hover:opacity-100 group-hover:mb-0 transition ease-out duration-[160ms]">
                                            <a class="font-poppins font-semibold text-3xl lg:text-4xl tracking-[0.5px] portfolio-stroke-text transition-all ease-linear duration-100" href="portfolio-single.html">Project Title</a>
                                        </div>
                                    </div>
                                </div>
                                <div class="portfolio-item category-1">
                                    <div class="relative overflow-hidden group rounded-lg after:content-[''] after:absolute after:top-0 after:left-0 after:w-full after:h-full after:bg-gradient-to-t after:from-black/30 after:to-transparent after:opacity-0 after:transition after:ease-out after:duration-[160ms] hover:after:opacity-100 category-1">
                                        <img class="transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.4px]" src="assets/images/portfolio-img.png" alt="" />
                                        <div class="z-[1] absolute top-4 right-4 bg-black/20 px-4 py-2 rounded-full backdrop-blur-[5px] text-white font-mono font-normal text-sm uppercase tracking-[0.5px]">
                                            Category
                                        </div>
                                        <div class="z-[1] absolute bottom-0 left-0 w-full px-7 pb-6 invisible opacity-0 translate-y-2 group-hover:translate-y-0 group-hover:visible group-hover:opacity-100 group-hover:mb-0 transition ease-out duration-[160ms]">
                                            <a class="font-poppins font-semibold text-3xl lg:text-4xl tracking-[0.5px] portfolio-stroke-text transition-all ease-linear duration-100" href="portfolio-single.html">Project Title</a>
                                        </div>
                                    </div>
                                </div>
                                <div class="portfolio-item category-2">
                                    <div class="relative overflow-hidden group rounded-lg after:content-[''] after:absolute after:top-0 after:left-0 after:w-full after:h-full after:bg-gradient-to-t after:from-black/30 after:to-transparent after:opacity-0 after:transition after:ease-out after:duration-[160ms] hover:after:opacity-100 category-1">
                                        <img class="transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.4px]" src="assets/images/portfolio-img.png" alt="" />
                                        <div class="z-[1] absolute top-4 right-4 bg-black/20 px-4 py-2 rounded-full backdrop-blur-[5px] text-white font-mono font-normal text-sm uppercase tracking-[0.5px]">
                                            Category
                                        </div>
                                        <div class="z-[1] absolute bottom-0 left-0 w-full px-7 pb-6 invisible opacity-0 translate-y-2 group-hover:translate-y-0 group-hover:visible group-hover:opacity-100 group-hover:mb-0 transition ease-out duration-[160ms]">
                                            <a class="font-poppins font-semibold text-3xl lg:text-4xl tracking-[0.5px] portfolio-stroke-text transition-all ease-linear duration-100" href="portfolio-single.html">Project Title</a>
                                        </div>
                                    </div>
                                </div>

                            </div>
                        </div>
                    </div>
                    <div id="services" class="section bg-white dark:bg-boxDark rounded-lg px-6 py-8 md:px-8 md:py-10 lg:p-12 shadow-sectionBoxShadow hover:shadow-sectionBoxShadowHover transition ease-out duration-[160ms]">
                        <div class="md:w-4/5 lg:w-3/4">
                            <h6 class="font-mono font-medium uppercase text-sm tracking-wider relative pt-4 mb-5 dark:text-white before:content-['//'] before:pr-2 after:content-[attr(data-backdrop-text)] after:absolute after:top-0 after:left-0 after:font-poppins after:font-bold after:uppercase after:text-4xl after:opacity-15" data-backdrop-text="Services">Services</h6>
                            <h2 class="text-3xl lg:text-4xl font-poppins font-semibold mb-3 lg:mb-4 dark:text-white">What I Do</h2>
                            <p class="text-pColor dark:text-white/70">Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore</p>
                        </div>
                        <div class="mt-6 lg:mt-10 divide-y divide-dashed divide-black/20">
                            <div class="md:flex py-6 lg:py-8 space-y-2.5 md:space-y-0">
                                <div class="md:min-w-[90px]">
                                    <span class="font-mono font-semibold text-5xl stroke-text">01</span>
                                </div>
                                <div class="md:min-w-[270px] dark:text-white">
                                    <i class="bi bi-code-slash text-2xl mb-2"></i>
                                    <div class="font-mono font-medium text-sm uppercase tracking-[0.5px]">Web &amp; Mobile Development</div>
                                </div>
                                <div>
                                    <p class="text-pColor dark:text-white/70 leading-7">Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat</p>
                                </div>
                            </div>
                            <div class="md:flex py-6 lg:py-8 space-y-2.5 md:space-y-0">
                                <div class="md:min-w-[90px]">
                                    <span class="font-mono font-semibold text-5xl stroke-text">02</span>
                                </div>
                                <div class="md:min-w-[270px] dark:text-white">
                                    <i class="bi bi-laptop text-2xl mb-2"></i>
                                    <div class="font-mono font-medium text-sm uppercase tracking-[0.5px]">Digital Marketing</div>
                                </div>
                                <div>
                                    <p class="text-pColor dark:text-white/70 leading-7">Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna</p>
                                </div>
                            </div>
                            <div class="md:flex py-6 lg:py-8 space-y-2.5 md:space-y-0">
                                <div class="md:min-w-[90px]">
                                    <span class="font-mono font-semibold text-5xl stroke-text">03</span>
                                </div>
                                <div class="md:min-w-[270px] dark:text-white">
                                    <i class="bi bi-gear text-2xl mb-2"></i>
                                    <div class="font-mono font-medium text-sm uppercase tracking-[0.5px]">Branding &amp; Strategy</div>
                                </div>
                                <div>
                                    <p class="text-pColor dark:text-white/70 leading-7">Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat</p>
                                </div>
                            </div>
                            <div class="md:flex py-6 lg:py-8 space-y-2.5 md:space-y-0">
                                <div class="md:min-w-[90px]">
                                    <span class="font-mono font-semibold text-5xl stroke-text">04</span>
                                </div>
                                <div class="md:min-w-[270px] dark:text-white">
                                    <i class="bi bi-person text-2xl mb-2"></i>
                                    <div class="font-mono font-medium text-sm uppercase tracking-[0.5px]">User Testing &amp; Personas</div>
                                </div>
                                <div>
                                    <p class="text-pColor dark:text-white/70 leading-7">Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna</p>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div id="resume" class="section bg-white dark:bg-boxDark rounded-lg px-6 py-8 md:px-8 md:py-10 lg:p-12 shadow-sectionBoxShadow hover:shadow-sectionBoxShadowHover transition ease-out duration-[160ms]">
                        <div class="md:w-4/5 lg:w-3/4">
                            <h6 class="font-mono font-medium uppercase text-sm tracking-wider relative pt-4 mb-5 dark:text-white before:content-['//'] before:pr-2 after:content-[attr(data-backdrop-text)] after:absolute after:top-0 after:left-0 after:font-poppins after:font-bold after:uppercase after:text-4xl after:opacity-15" data-backdrop-text="Resume">Resume</h6>
                            <h2 class="text-3xl lg:text-4xl font-poppins font-semibold mb-3 lg:mb-4 dark:text-white">Education & Experience</h2>
                            <p class="text-pColor dark:text-white/70">Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore</p>
                        </div>
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 md:gap-8 lg:gap-10 mt-6 lg:mt-12">
                            <div class="relative pl-5 space-y-7 before:content-[''] before:absolute before:top-0 before:left-0 before:w-[1px] before:h-full before:border-l before:border-black/20 dark:before:border-white/20 before:border-dashed">
                                <div class="text-3xl dark:text-white">
                                    <i class="bi bi-mortarboard"></i>
                                </div>
                                <div class="group">
                                    <div class="relative inline-block px-4 py-2 rounded-full border border-black/20 dark:border-white/20 border-dashed font-mono font-medium uppercase text-sm tracking-[0.5px] text-pColor dark:text-white/70 group-hover:text-black dark:group-hover:text-white transition ease-linear duration-100 before:content-[''] before:absolute before:top-1/2 before:left-[-20px] before:w-[20px] before:h-[1px] before:border-t before:border-black/20 dark:before:border-white/20 before:border-dashed after:content-[''] after:absolute after:top-1/2 after:left-[-22px] after:-translate-y-1/2 after:bg-black dark:after:bg-white after:w-[5px] after:h-[5px] after:rounded-full">
                                        2020 - 2023
                                    </div>
                                    <h4 class="font-poppins font-medium text-lg lg:text-xl mt-2 mb-1 lg:mt-3 lg:mb-2 dark:text-white">Bachelor Degree of Business</h4>
                                    <span class="text-pColor dark:text-white/70">@University of Business</span>
                                </div>
                                <div class="group">
                                    <div class="relative inline-block px-4 py-2 rounded-full border border-black/20 dark:border-white/20 border-dashed font-mono font-medium uppercase text-sm tracking-[0.5px] text-pColor dark:text-white/70 group-hover:text-black dark:group-hover:text-white transition ease-linear duration-100 before:content-[''] before:absolute before:top-1/2 before:left-[-20px] before:w-[20px] before:h-[1px] before:border-t before:border-black/20 dark:before:border-white/20 before:border-dashed after:content-[''] after:absolute after:top-1/2 after:left-[-22px] after:-translate-y-1/2 after:bg-black dark:after:bg-white after:w-[5px] after:h-[5px] after:rounded-full">
                                        2018 - 2020
                                    </div>
                                    <h4 class="font-poppins font-medium text-lg lg:text-xl mt-2 mb-1 lg:mt-3 lg:mb-2 dark:text-white">Master Degree of Design</h4>
                                    <span class="text-pColor dark:text-white/70">@University of IT</span>
                                </div>
                                <div class="group">
                                    <div class="relative inline-block px-4 py-2 rounded-full border border-black/20 dark:border-white/20 border-dashed font-mono font-medium uppercase text-sm tracking-[0.5px] text-pColor dark:text-white/70 group-hover:text-black dark:group-hover:text-white transition ease-linear duration-100 before:content-[''] before:absolute before:top-1/2 before:left-[-20px] before:w-[20px] before:h-[1px] before:border-t before:border-black/20 dark:before:border-white/20 before:border-dashed after:content-[''] after:absolute after:top-1/2 after:left-[-22px] after:-translate-y-1/2 after:bg-black dark:after:bg-white after:w-[5px] after:h-[5px] after:rounded-full">
                                        2014 - 2018
                                    </div>
                                    <h4 class="font-poppins font-medium text-lg lg:text-xl mt-2 mb-1 lg:mt-3 lg:mb-2 dark:text-white">Bachelor Degree of Design</h4>
                                    <span class="text-pColor dark:text-white/70">@University of Design</span>
                                </div>
                            </div>
                            <div class="relative pl-5 space-y-7 before:content-[''] before:absolute before:top-0 before:left-0 before:w-[1px] before:h-full before:border-l before:border-black/20 dark:before:border-white/20 before:border-dashed">
                                <div class="text-3xl dark:text-white">
                                    <i class="bi bi-briefcase"></i>
                                </div>
                                <div class="group">
                                    <div class="relative inline-block px-4 py-2 rounded-full border border-black/20 dark:border-white/20 border-dashed font-mono font-medium uppercase text-sm tracking-[0.5px] text-pColor dark:text-white/70 group-hover:text-black dark:group-hover:text-white transition ease-linear duration-100 before:content-[''] before:absolute before:top-1/2 before:left-[-20px] before:w-[20px] before:h-[1px] before:border-t before:border-black/20 dark:before:border-white/20 before:border-dashed after:content-[''] after:absolute after:top-1/2 after:left-[-22px] after:-translate-y-1/2 after:bg-black dark:after:bg-white after:w-[5px] after:h-[5px] after:rounded-full">
                                        2020 - Present
                                    </div>
                                    <h4 class="font-poppins font-medium text-lg lg:text-xl mt-2 mb-1 lg:mt-3 lg:mb-2 dark:text-white">Director of Operations</h4>
                                    <span class="text-pColor dark:text-white/70">@FlaTheme</span>
                                </div>
                                <div class="group">
                                    <div class="relative inline-block px-4 py-2 rounded-full border border-black/20 dark:border-white/20 border-dashed font-mono font-medium uppercase text-sm tracking-[0.5px] text-pColor dark:text-white/70 group-hover:text-black dark:group-hover:text-white transition ease-linear duration-100 before:content-[''] before:absolute before:top-1/2 before:left-[-20px] before:w-[20px] before:h-[1px] before:border-t before:border-black/20 dark:before:border-white/20 before:border-dashed after:content-[''] after:absolute after:top-1/2 after:left-[-22px] after:-translate-y-1/2 after:bg-black dark:after:bg-white after:w-[5px] after:h-[5px] after:rounded-full">
                                        2018 - 2020
                                    </div>
                                    <h4 class="font-poppins font-medium text-lg lg:text-xl mt-2 mb-1 lg:mt-3 lg:mb-2 dark:text-white">Senior Designer</h4>
                                    <span class="text-pColor dark:text-white/70">@FlaTheme</span>
                                </div>
                                <div class="group">
                                    <div class="relative inline-block px-4 py-2 rounded-full border border-black/20 dark:border-white/20 border-dashed font-mono font-medium uppercase text-sm tracking-[0.5px] text-pColor dark:text-white/70 group-hover:text-black dark:group-hover:text-white transition ease-linear duration-100 before:content-[''] before:absolute before:top-1/2 before:left-[-20px] before:w-[20px] before:h-[1px] before:border-t before:border-black/20 dark:before:border-white/20 before:border-dashed after:content-[''] after:absolute after:top-1/2 after:left-[-22px] after:-translate-y-1/2 after:bg-black dark:after:bg-white after:w-[5px] after:h-[5px] after:rounded-full">
                                        2015 - 2018
                                    </div>
                                    <h4 class="font-poppins font-medium text-lg lg:text-xl mt-2 mb-1 lg:mt-3 lg:mb-2 dark:text-white">UI & UX Designer</h4>
                                    <span class="text-pColor dark:text-white/70">@FlaTheme</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div id="blog" class="section bg-white dark:bg-boxDark rounded-lg px-6 py-8 md:px-8 md:py-10 lg:p-12 shadow-sectionBoxShadow hover:shadow-sectionBoxShadowHover transition ease-out duration-[160ms]">
                        <div class="md:w-4/5 lg:w-3/4">
                            <h6 class="font-mono font-medium uppercase text-sm tracking-wider relative pt-4 mb-5 dark:text-white before:content-['//'] before:pr-2 after:content-[attr(data-backdrop-text)] after:absolute after:top-0 after:left-0 after:font-poppins after:font-bold after:uppercase after:text-4xl after:opacity-15" data-backdrop-text="Blog">Blog</h6>
                            <h2 class="text-3xl lg:text-4xl font-poppins font-semibold mb-3 lg:mb-4 dark:text-white">Latest Blog Posts</h2>
                            <p class="text-pColor dark:text-white/70">Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore</p>
                        </div>

                        <div class="mt-6 lg:mt-12 space-y-8 md:space-y-6">
                            <div class="md:flex md:items-center">
                                <div class="overflow-hidden relative rounded-lg group">
                                    <a href="blog-single.html">
                                        <img class="md:max-w-[340px] transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.5px]" src="assets/images/blog-img.png" alt="" />
                                        <div class="absolute top-4 left-4 bg-black/20 px-4 py-2 rounded-full text-white backdrop-blur-[5px] font-mono font-normal uppercase text-sm tracking-[0.5px]">
                                            Category
                                        </div>
                                    </a>
                                </div>
                                <div class="mt-5 md:pl-7 md:mt-0">
                                    <span class="text-pColor dark:text-white/70">Posted on Nov 20</span>
                                    <h3 class="font-poppins font-semibold text-2xl lg:text-3xl mt-2"><a class="hover:underline dark:text-white" href="blog-single.html">Blog Post Title</a></h3>
                                    <a class="inline-block border border-black border-dashed rounded-full px-6 py-3 mt-3 lg:mt-4 font-mono text-sm hover:bg-black hover:text-white transition ease-out duration-[120ms] dark:text-white dark:border-white dark:hover:bg-white dark:hover:text-black" href="blog-single.html">Read More</a>
                                </div>
                            </div>
                        </div>
                        <div class="mt-6 lg:mt-12 space-y-8 md:space-y-6">
                            <div class="md:flex md:items-center">
                                <div class="overflow-hidden relative rounded-lg group">
                                    <a href="blog-single.html">
                                        <img class="md:max-w-[340px] transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.5px]" src="assets/images/blog-img.png" alt="" />
                                        <div class="absolute top-4 left-4 bg-black/20 px-4 py-2 rounded-full text-white backdrop-blur-[5px] font-mono font-normal uppercase text-sm tracking-[0.5px]">
                                            Category
                                        </div>
                                    </a>
                                </div>
                                <div class="mt-5 md:pl-7 md:mt-0">
                                    <span class="text-pColor dark:text-white/70">Posted on Nov 20</span>
                                    <h3 class="font-poppins font-semibold text-2xl lg:text-3xl mt-2"><a class="hover:underline dark:text-white" href="blog-single.html">Blog Post Title</a></h3>
                                    <a class="inline-block border border-black border-dashed rounded-full px-6 py-3 mt-3 lg:mt-4 font-mono text-sm hover:bg-black hover:text-white transition ease-out duration-[120ms] dark:text-white dark:border-white dark:hover:bg-white dark:hover:text-black" href="blog-single.html">Read More</a>
                                </div>
                            </div>
                        </div>
                        <div class="mt-6 lg:mt-12 space-y-8 md:space-y-6">
                            <div class="md:flex md:items-center">
                                <div class="overflow-hidden relative rounded-lg group">
                                    <a href="blog-single.html">
                                        <img class="md:max-w-[340px] transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.5px]" src="assets/images/blog-img.png" alt="" />
                                        <div class="absolute top-4 left-4 bg-black/20 px-4 py-2 rounded-full text-white backdrop-blur-[5px] font-mono font-normal uppercase text-sm tracking-[0.5px]">
                                            Category
                                        </div>
                                    </a>
                                </div>
                                <div class="mt-5 md:pl-7 md:mt-0">
                                    <span class="text-pColor dark:text-white/70">Posted on Nov 20</span>
                                    <h3 class="font-poppins font-semibold text-2xl lg:text-3xl mt-2"><a class="hover:underline dark:text-white" href="blog-single.html">Blog Post Title</a></h3>
                                    <a class="inline-block border border-black border-dashed rounded-full px-6 py-3 mt-3 lg:mt-4 font-mono text-sm hover:bg-black hover:text-white transition ease-out duration-[120ms] dark:text-white dark:border-white dark:hover:bg-white dark:hover:text-black" href="blog-single.html">Read More</a>
                                </div>
                            </div>
                        </div>
                        <div class="mt-6 lg:mt-12 space-y-8 md:space-y-6">
                            <div class="md:flex md:items-center">
                                <div class="overflow-hidden relative rounded-lg group">
                                    <a href="blog-single.html">
                                        <img class="md:max-w-[340px] transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.5px]" src="assets/images/blog-img.png" alt="" />
                                        <div class="absolute top-4 left-4 bg-black/20 px-4 py-2 rounded-full text-white backdrop-blur-[5px] font-mono font-normal uppercase text-sm tracking-[0.5px]">
                                            Category
                                        </div>
                                    </a>
                                </div>
                                <div class="mt-5 md:pl-7 md:mt-0">
                                    <span class="text-pColor dark:text-white/70">Posted on Nov 20</span>
                                    <h3 class="font-poppins font-semibold text-2xl lg:text-3xl mt-2"><a class="hover:underline dark:text-white" href="blog-single.html">Blog Post Title</a></h3>
                                    <a class="inline-block border border-black border-dashed rounded-full px-6 py-3 mt-3 lg:mt-4 font-mono text-sm hover:bg-black hover:text-white transition ease-out duration-[120ms] dark:text-white dark:border-white dark:hover:bg-white dark:hover:text-black" href="blog-single.html">Read More</a>
                                </div>
                            </div>
                        </div>

                    </div>
                </div>
            </div>
        </div>

    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
