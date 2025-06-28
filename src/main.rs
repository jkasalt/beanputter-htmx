use dioxus::{ prelude::*};

static CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[derive(Clone)]
struct TitleState(String);

#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        div { id: "title",
                h1 { "{title.0}" }
        }
    }
}

#[component]
fn DogView() -> Element {
    let img_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
    rsx! {
        div { id: "dogview",
            img { src: "{img_src}"}
        }
        div { id: "buttons",
            button { id: "skip", "skip" }
            button { id: "save", "save!" }
        }
    }
}

#[component]
fn App() -> Element {
    let title = use_context_provider(|| TitleState(String::from("HotDog! 🌭")));
    rsx! {
        document::Stylesheet { href: CSS }
        Title {}
        DogView {}
    }
}
