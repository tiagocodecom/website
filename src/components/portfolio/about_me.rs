use leptos::prelude::*;

use crate::components::*;

#[component]
pub fn AboutMe() -> impl IntoView {

    view! {
        <Container id="about">
            <div class="lg:flex lg:space-x-10">
                <div class="relative h-fit mb-6">
                    <img class="min-w-52 min-h-52 max-w-64 max-h-64 rounded-full" src="assets/images/hero-avatar.jpg" alt="" />
                    <div class="absolute bottom-4 left-2 bg-black/30 dark:bg-black/60 px-4 py-2 rounded-full shadow-avatarText backdrop-blur-[5px] text-white font-mono font-normal uppercase text-sm tracking-wider">
                        <span class="typer" id="typer1" data-words="Hi There!, I'm Christina" data-colors="white" data-delay="50" data-deleteDelay="1500"></span><span class="cursor" data-owner="typer1"></span>
                    </div>
                </div>
                <div>
                    <ContainerTitle backdrop_text="About Me">"About Me"</ContainerTitle>
                    <ContainerSubtitle>"Backend Developer PHP"</ContainerSubtitle>
                    <ContainerDescription>
                        "Lorem ipsum dolor sit amet, consectetur adipisicing elita"
                    </ContainerDescription>
                    <ul class="space-y-3 mb-4">
                        <li class="list-none inline-block px-4 py-2 me-2 rounded-full border border-black/20 dark:border-white/30 border-dashed text-pColor hover:text-black dark:text-white/70 dark:hover:text-white transition ease-linear duration-100">
                            <i class="bi bi-code-slash pe-1"></i>
                            PHP
                            <div class="inline-block font-mono text-sm">"("<span class="counter">85</span>%")"</div>
                        </li>
                        <li class="list-none inline-block px-4 py-2 me-2 rounded-full border border-black/20 dark:border-white/30 border-dashed text-pColor hover:text-black dark:text-white/70 dark:hover:text-white transition ease-linear duration-100">
                            <i class="bi bi-code-slash pe-1"></i>
                            Laravel
                            <div class="inline-block font-mono text-sm">"("<span class="counter">94</span>%")"</div>
                        </li>
                        <li class="list-none inline-block px-4 py-2 me-2 rounded-full border border-black/20 dark:border-white/30 border-dashed text-pColor hover:text-black dark:text-white/70 dark:hover:text-white transition ease-linear duration-100">
                            <i class="bi bi-code-slash pe-1"></i>
                            Drupal
                            <div class="inline-block font-mono text-sm">"("<span class="counter">98</span>%")"</div>
                        </li>
                    </ul>
                </div>
            </div>
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-5 mt-8">
                <div class="flex items-center">
                    <div class="pe-2">
                        <div class="font-mono font-semibold text-6xl stroke-text"><span class="counter">6</span></div>
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
        </Container>
    }
}