use leptos::prelude::*;
use crate::components::*;
use crate::components::portfolio::*;

#[component]
pub fn LatestWork() -> impl IntoView {
    view! {
        <Container id="portfolio">
            <div class="md:w-4/5 lg:w-3/4">
                <ContainerSubtitle backdrop_text="Projects">Projects</ContainerSubtitle>
                <ContainerTitle>My Latest Works</ContainerTitle>
                <ContainerDescription>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore</ContainerDescription>
            </div>
            <div class="mt-6 lg:mt-12">
                <div class="portfolio-grid grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 mt-6">
                    <LatestWorkCard>
                        <LatestWorkCardImage src="/assets/images/portfolio-img.png" />
                        <LatestWorkCardLink href="#">
                            "Project Title 1"
                        </LatestWorkCardLink>
                    </LatestWorkCard>
                    <LatestWorkCard>
                        <LatestWorkCardImage src="/assets/images/portfolio-img.png" />
                        <LatestWorkCardLink href="#">
                            "Project Title 2"
                        </LatestWorkCardLink>
                    </LatestWorkCard>
                    <LatestWorkCard>
                        <LatestWorkCardImage src="/assets/images/portfolio-img.png" />
                        <LatestWorkCardLink href="#">
                            "Project Title 3"
                        </LatestWorkCardLink>
                    </LatestWorkCard>
                </div>
            </div>
        </Container>
    }
}