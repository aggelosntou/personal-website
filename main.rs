use yew::prelude::*;
use yew::Renderer;

#[function_component(Hero)]
fn hero() -> Html {
    html! {
        <section class="text-center py-20">
            <h1 class="text-4xl font-bold mb-4">{ "Hi, I'm Aggelos" }</h1>
            <p class="text-lg text-gray-600">{ "I am a Math Major and tech/AI Enthusiast" }</p>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct ProjectProps {
    title: String,
    description: String,
    link: String,
}

#[function_component(PersonalSection)]
fn personal_section() -> Html {
    html! {
        <section class="py-12 px-6 max-w-2xl mx-auto">
            <h2 class="text-2xl font-bold mb-4">{ "Personal" }</h2>
            <ul class="list-disc list-inside space-y-2 text-gray-700">
                <li><a href="/about" class="text-blue-600 hover:underline">{ "About Me" }</a></li>
                <li><a href="/books" class="text-blue-600 hover:underline">{ "Books I’ve Read" }</a></li>
                <li><a href="https://linkedin.com/in/YOUR_ID" class="text-blue-600 hover:underline" target="_blank">{ "LinkedIn" }</a></li>
                <li><a href="https://github.com/aggelosntou" class="text-blue-600 hover:underline" target="_blank">{ "GitHub" }</a></li>
            </ul>
        </section>
    }
}

#[function_component(Project)]
fn project(props: &ProjectProps) -> Html {
    html! {
        <a href={props.link.clone()} target="_blank" class="block border p-6 rounded-lg hover:bg-gray-100 transition">
            <h2 class="text-xl font-semibold mb-2">{ &props.title }</h2>
            <p class="text-gray-600">{ &props.description }</p>
        </a>
    }
}

#[function_component(Projects)]
fn projects() -> Html {
    html! {
        <section class="py-12 px-6 max-w-3xl mx-auto">
            <h2 class="text-2xl font-bold mb-6 text-center">{ "Projects" }</h2>
            <div class="grid gap-4">
                <Project
                    title="Statistical Analysis of Global Economic Indicators"
                    description="A comprehensive analysis of global economic indicators using R."
                    link="https://github.com/aggelosntou"
                />
                <Project
                    title="Numerical Optimization Algorithms"
                    description="Class Homework on numerical optimization algorithms."
                    link="https://github.com/aggelosntou"
                />
            </div>
        </section>
    }
}




#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="font-sans text-gray-800">
            <Hero />
            <PersonalSection />
            <Projects />
            <RecentPosts />
        </div>
    }
}


fn main() {
    Renderer::<App>::new().render();
}

