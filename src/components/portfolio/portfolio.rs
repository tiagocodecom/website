use leptos::prelude::*;

use crate::components::*;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <Container id="blog">
            <div class="md:w-4/5 lg:w-3/4">
                <ContainerTitle backdrop_text="Blog">Blog</ContainerTitle>
                <ContainerSubtitle>Latest Blog Posts</ContainerSubtitle>
                <ContainerDescription>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore</ContainerDescription>
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
        </Container>
    }
}