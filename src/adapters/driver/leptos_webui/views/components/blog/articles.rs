use leptos::prelude::*;

use crate::adapters::driver::leptos_webui::views::components::common::*;
use crate::application::domain::article::{Article, Category};
use crate::application::domain::common::Image;

#[component]
pub fn ArticlesSection(categories: Vec<Category>, articles: Vec<Article>) -> impl IntoView {
    view! {
        <DefaultContainer>
            <div>
                <ContainerSubtitle text="Blog".to_string() />
                <ContainerH1Title text="Articles about tech".to_string() />
                <div class="py-6">
                    <Pill text="Programming".into() emoji="🖥️".into()/>
                    <Pill text="Embedded".into() emoji="🤖".into() />
                    <Pill text="Keyboards".into() emoji="⌨️".into()/>
                    <Pill text="Blockchain".into() emoji="🪙".into() />
                </div>
            </div>
            <div class="grid grid-cols-1 xl:grid-cols-2 gap-4 mt-6 lg:mt-3">
                {articles
                    .into_iter()
                    .map(|a| {
                        view! {
                            <ArticleCard
                                date=a.created_at().to_string_with_format("%b %d, %Y")
                                title=a.title().to_string()
                                summary=a.summary().to_string()
                                slug=a.slug().to_string()
                                category=a.category().clone()
                                thumbnail=a.thumbnail().clone()
                            />
                        }
                    }).collect_view()}
            </div>
        </DefaultContainer>
    }
}

#[component]
pub fn ArticleCard(
    date: String,
    slug: String,
    title: String,
    summary: String,
    thumbnail: Image,
    category: Category,
) -> impl IntoView {
    view! {
        <div class="md:flex md:items-start md:justify-center mt-8">
            <div class="overflow-hidden relative rounded-lg group  flex-shrink-0">
                <img
                    src=thumbnail.url().to_string()
                    alt=thumbnail.alt().to_string()
                    class="w-full transition ease-custom duration-500 group-hover:scale-105 group-hover:blur-[1.5px]"
                />
                <div class="absolute top-4 left-4 bg-black/50 px-4 py-2 rounded-full text-white backdrop-blur-[5px] font-mono font-normal uppercase text-sm tracking-[0.5px]">
                    {category.title().to_string()}
                    <span class="ml-2">{category.emoji().to_string()}</span>
                </div>
            </div>
            <div class="md:pl-7 md:mt-0  flex-grow">
                <span class="text-pColor dark:text-white/70">{date}</span>
                <h3 class="font-poppins font-semibold text-lg mt-2">{title}</h3>
                <p class="text-pColor dark:text-white/70">{
                    if summary.chars().count() > 110 {
                        summary.chars().take(110).collect::<String>() + "..."
                    } else {
                        summary
                    }
                }</p>
                <a href=slug target="_self" class="inline-block border border-black border-dashed rounded-full px-6 py-3 mt-3 lg:mt-4 font-mono text-sm hover:bg-black hover:text-white transition ease-out duration-[120ms] dark:text-white dark:border-white dark:hover:bg-white dark:hover:text-black">
                    Read More
                </a>
            </div>
        </div>
    }
}
