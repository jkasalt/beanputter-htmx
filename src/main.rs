use std::collections::HashMap;

use dioxus::prelude::*;
use transaction::{SingleTransaction, SingleTransactionView, SingleTransactionViewProps};

mod csv;
mod transaction;

static TAILWIND: Asset = asset!("/assets/tailwind.css");

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
            img { src: "{img_src}" }
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
    let transaction_csv = use_hook(|| {
        crate::csv::read_ubs_csv(r#"
Date de transaction;Heure de transaction;Date de comptabilisation;Date de valeur;Monnaie;Débit;Crédit;Sous-montant;Solde;N° de transaction;Description1;Description2;Description3;Notes de bas de page;
2025-03-14;;2025-03-14;2025-03-14;CHF;2.40;;;39942.6;1234567890123456;"Bing bong ullabong";"Haha";;;
2025-03-13;;2025-03-13;2025-03-13;CHF;;60.00;;39940.2;0000067890123456;"Dun dun dun";"Hoho";;;"#).unwrap()
    });

    let transaction_views_props = use_hook(|| {
        transaction_csv
            .into_iter()
            .map(SingleTransaction::from)
            .enumerate()
            .map(|(i, t)| SingleTransactionViewProps::from_model_with_id(t, i))
            .collect::<Vec<_>>()
    });

    let selection = use_signal(|| {
        transaction_views_props
            .iter()
            .map(|tv| (tv.id, false))
            .collect::<HashMap<_, _>>()
    });

    let selected = use_memo(move || {
        selection
            .read()
            .iter()
            .filter_map(|(id, v)| v.then_some(*id))
            .collect::<Vec<_>>()
    });

    use_effect(move || println!("{:?}", selection.read()));

    rsx! {
        document::Stylesheet { href: TAILWIND }
        // Title {}
        // DogView {}
        {format!("{selected:?}")}
        {
            transaction_views_props
                .iter()
                .map(|t| {
                    let id = t.id;
                    let selected = *selection.read().get(&id).unwrap_or(&false);
                    let label = if selected { "Deselect"} else {"Select"};
                    rsx! {
                        button {
                            onclick: move |_| {
                                selection.clone().write().entry(id).and_modify(|v| *v = !*v).or_insert(true);
                            },
                            "{label}" SingleTransactionView { ..t.clone() }
                        }
                    }
                })
        }
    }
}
