use leptos::prelude::*;

#[component]
pub fn SidebarMenu() -> impl IntoView {
    view! {
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
                        href="#portfolio"
                    >
                        <span class="hidden lg:inline-block">Projects</span><span class="lg:hidden">P</span>
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
                        <span class="hidden lg:inline-block">Skills</span><span class="lg:hidden">S</span>
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
            </ul>
        </div>
    }
}