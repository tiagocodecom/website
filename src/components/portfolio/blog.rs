use leptos::prelude::*;
use crate::components::*;
use crate::components::portfolio::*;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <Container id="blog">
            <div class="md:w-4/5 lg:w-3/4">
                <ContainerSubtitle backdrop_text="Blog">Blog</ContainerSubtitle>
                <ContainerTitle>Latest Blog Posts</ContainerTitle>
                <ContainerDescription>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore</ContainerDescription>
            </div>
            <div class="mt-6 lg:mt-12 space-y-8 md:space-y-6">
                <BlogCard>
                    <BlogCardImage src="/assets/images/blog-img.png" alt="Example blog" overlay_text="Category"/>
                    <BlogCardContent>
                        <BlogCardDate>Posted on Nov 20</BlogCardDate>
                        <BlogCardTitle>Blog Post Title</BlogCardTitle>
                        <BlogCardDescription>Lorem ipsum dolor sit amet, consectetur adipisicing elita</BlogCardDescription>
                        <BlogCardLink href="#" />
                    </BlogCardContent>
                </BlogCard>
                <BlogCard>
                    <BlogCardImage src="/assets/images/blog-img.png" alt="Example blog" overlay_text="Category"/>
                    <BlogCardContent>
                        <BlogCardDate>Posted on Nov 20</BlogCardDate>
                        <BlogCardTitle>Blog Post Title</BlogCardTitle>
                        <BlogCardDescription>Lorem ipsum dolor sit amet, consectetur adipisicing elita</BlogCardDescription>
                        <BlogCardLink href="#" />
                    </BlogCardContent>
                </BlogCard>
            </div>
        </Container>
    }
}