use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::blog::components::card::*;
use crate::adapters::driver::leptos_webui::views::common::components::Pill;
use crate::adapters::driver::leptos_webui::views::common::components::*;
use crate::adapters::driver::leptos_webui::views::common::layouts::BasicLayout;

#[component]
pub fn BlogPage() -> impl IntoView {
    view! {
        <BasicLayout>
            <div class="lg:flex justify-center space-y-6 lg:space-y-0 lg:space-x-1">
                <div class="lg:w-3/4">
                    <DefaultContainer>
                        <div>
                            <ContainerSubtitle text="Blog".to_string() />
                            <ContainerH1Title text="Latest Article".to_string() />
                            <div class="py-6">
                                <Pill text="Programming".into() emoji="🖥️".into()/>
                                <Pill text="Embedded".into() emoji="🤖".into() />
                                <Pill text="Keyboards".into() emoji="⌨️".into()/>
                                <Pill text="Blockchain".into() emoji="🪙".into() />
                            </div>
                        </div>
                        <div class="mt-6 lg:mt-12 space-y-8 md:space-y-6">
                            <BlogCard>
                                <BlogCardImage
                                    src="/assets/images/blog-img.png".into()
                                    alt="Example blog".into()
                                    overlay_text="Category".into()
                                />
                                <BlogCardContent>
                                    <BlogCardDate text="Posted on Nov 20".into() />
                                    <BlogCardTitle text="Blog Post Title".into() />
                                    <BlogCardDescription text="Lorem ipsum dolor sit amet, consectetur adipisicing elita".into() />
                                    <BlogCardLink href="#".into() />
                                </BlogCardContent>
                            </BlogCard>
                            <BlogCard>
                                <BlogCardImage
                                    src="/assets/images/blog-img.png".into()
                                    alt="Example blog".into()
                                    overlay_text="Category".into()
                                />
                                <BlogCardContent>
                                    <BlogCardDate text="Posted on Nov 20".into() />
                                    <BlogCardTitle text="Blog Post Title".into() />
                                    <BlogCardDescription text="Lorem ipsum dolor sit amet, consectetur adipisicing elita".into() />
                                    <BlogCardLink href="#".into() />
                                </BlogCardContent>
                            </BlogCard>
                            <BlogCard>
                                <BlogCardImage
                                    src="/assets/images/blog-img.png".into()
                                    alt="Example blog".into()
                                    overlay_text="Category".into()
                                />
                                <BlogCardContent>
                                    <BlogCardDate text="Posted on Nov 20".into() />
                                    <BlogCardTitle text="Blog Post Title".into() />
                                    <BlogCardDescription text="Lorem ipsum dolor sit amet, consectetur adipisicing elita".into() />
                                    <BlogCardLink href="#".into() />
                                </BlogCardContent>
                            </BlogCard>
                        </div>
                    </DefaultContainer>
                </div>
                <CustomContainer class="hidden lg:block min-h-full lg:w-1/4 px-8 md:px-8 py-8 md:py-2 lg:py-12 bg-white backdrop-blur-[5px] lg:backdrop-blur-none".into()>
                    <div class="mt-6 space">
                        <ContainerH2Title text="Featured".to_string() />
                        <BlogCard>
                            <BlogCardImage
                                src="/assets/images/portfolio-img.png".into()
                                alt="Example blog".into()
                                overlay_text="Category".into()
                            />
                        </BlogCard>
                    </div>
                </CustomContainer>
            </div>
        </BasicLayout>
    }
}
