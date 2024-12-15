use leptos::prelude::*;
use crate::components::*;

#[component]
pub fn Resume() -> impl IntoView {
    view! {
        <Container id="resume">
            <div class="md:w-4/5 lg:w-3/4">
                <ContainerSubtitle backdrop_text="Resume">"Resume"</ContainerSubtitle>
                <ContainerTitle>Education & Experience</ContainerTitle>
                <ContainerDescription>Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore</ContainerDescription>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 md:gap-8 lg:gap-10 mt-6 lg:mt-12">
                <Timeline icon="bi bi-mortarboard">
                    <TimelineItem>
                        <TimelineDate>2020 - 2023</TimelineDate>
                        <TimelineTitle>Bachelor Degree of Business</TimelineTitle>
                        <TimelineSubtitle>@University of Business</TimelineSubtitle>
                    </TimelineItem>
                </Timeline>
                <Timeline icon="bi bi-briefcase">
                    <TimelineItem>
                        <TimelineDate>2022 - Present</TimelineDate>
                        <TimelineTitle>Backend developer</TimelineTitle>
                        <TimelineSubtitle>Videoslots</TimelineSubtitle>
                    </TimelineItem>
                    <TimelineItem>
                        <TimelineDate>2022 - Present</TimelineDate>
                        <TimelineTitle>Backend developer</TimelineTitle>
                        <TimelineSubtitle>Videoslots</TimelineSubtitle>
                    </TimelineItem>
                </Timeline>
            </div>
        </Container>
    }
}