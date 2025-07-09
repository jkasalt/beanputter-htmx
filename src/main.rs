use std::collections::HashMap;

use dioxus::prelude::*;
use transaction::{SingleTransaction, SingleTransactionView, SingleTransactionViewProps};

mod csv;
mod transaction;

static TAILWIND: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
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
        div { class: "flex flex-col",
            {
                transaction_views_props
                    .iter()
                    .map(|t| {
                        let id = t.id;
                        let selected = *selection.read().get(&id).unwrap_or(&false);
                        let common_class = "flex flex-row cursor-pointer gap-2";
                        let selected_class = if selected {
                            "bg-green-100"
                        } else {
                            "bg-gray-200"
                        };
                        let class = format!("{common_class} {selected_class}");
                        rsx! {
                            button {
                                class,
                                onclick: move |_| {
                                    selection.clone().write().entry(id).and_modify(|v| *v = !*v).or_insert(true);
                                },
                                input { r#type: "checkbox", checked: selected }
                                SingleTransactionView { ..t.clone() }
                            }
                        }
                    })
            }
        }
    }
}
