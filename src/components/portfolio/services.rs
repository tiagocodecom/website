use leptos::prelude::*;
use crate::components::*;

#[component]
pub fn Services() -> impl IntoView {
    view! {
        <Container id="services">
            <div class="md:w-4/5 lg:w-3/4">
                <ContainerSubtitle backdrop_text="Skills">"Skills"</ContainerSubtitle>
                <ContainerTitle>What I Do</ContainerTitle>
                <ContainerDescription>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore</ContainerDescription>
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
        </Container>
    }
}