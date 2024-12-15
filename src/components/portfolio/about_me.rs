use leptos::prelude::*;
use crate::components::*;

#[component]
pub fn AboutMe() -> impl IntoView {

    view! {
        <Container id="about">
            <div class="lg:flex lg:space-x-10">
                <div class="relative h-fit">
                    <div>
                        <img class="min-w-52 min-h-52 max-w-64 max-h-64 rounded-full" src="assets/images/hero-avatar.jpg" alt="" />
                    </div>
                    <div class="flex items-center justify-center">
                        <div class="pe-2">
                            <div class="font-mono font-semibold text-6xl stroke-text">
                                <span class="counter">6</span>
                            </div>
                        </div>
                        <div class="dark:text-white py-2">
                            <span class="block text-2xl font-normal mb-1">+</span>
                            <p class="font-mono font-medium text-sm uppercase tracking-[0.5px]">Years of Experience</p>
                        </div>
                    </div>
                </div>
                <div>
                    <ContainerSubtitle backdrop_text="About Me">"About Me"</ContainerSubtitle>
                    <ContainerTitle>"Backend Developer PHP"</ContainerTitle>
                    <ContainerDescription>
                        "Lorem ipsum dolor sit amet, consectetur adipisicing elita Lorem ipsum dolor \
                         sit amet, consectetur adipisicing elita Lorem ipsum dolor sit amet, \
                         sit amet, consectetur adipisicing elita Lorem ipsum dolor sit amet, \
                         sit amet, consectetur adipisicing elita Lorem ipsum dolor sit amet, \
                         consectetur adipisicing elita"
                    </ContainerDescription>
                    <div class="space-y-3 mb-4">
                        <Badge>"PHP"</Badge>
                        <Badge>"Laravel"</Badge>
                        <Badge>"Drupal"</Badge>
                        <Badge>"ReactJs"</Badge>
                        <Badge>"Rust"</Badge>
                        <Badge>"Linux"</Badge>
                        <Badge>"Fullstack"</Badge>
                    </div>
                </div>
            </div>
        </Container>
    }
}