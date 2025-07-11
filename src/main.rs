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
    let mut original_csv = use_signal(|| {
        String::from(
            r#"Numéro de compte:;0243 00517049.40;
IBAN:;CH18 0024 3243 5170 4940 F;
Du:;2025-03-01;
Au:;2025-03-14;
Solde initial:;86124.47;
Solde final:;84298.77;
Évaluation en:;CHF;
Nombre de transactions dans cette période:;27;

Date de transaction;Heure de transaction;Date de comptabilisation;Date de valeur;Monnaie;Débit;Crédit;Sous-montant;Solde;N° de transaction;Description1;Description2;Description3;Notes de bas de page;
2025-03-14;;2025-03-14;2025-03-14;CHF;2.40;;;39942.6;1234567890123456;"Bing bong ullabong";"Haha";;;
2025-03-13;;2025-03-13;2025-03-13;CHF;;60.00;;39940.2;0000067890123456;"Dun dun dun";"Hoho";;;"#,
        )
    });
    let transaction_views_props = use_memo(move || {
        crate::csv::read_ubs_csv(&original_csv.read())
            .unwrap()
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

    let select = |id| {
        move |_| {
            selection
                .clone()
                .write()
                .entry(id)
                .and_modify(|v| *v = !*v)
                .or_insert(true);
        }
    };

    use_effect(move || println!("{:?}", selection.read()));

    rsx! {
        document::Stylesheet { href: TAILWIND }
        div {
            class: "flex flex-row gap-2",
            label {
                for: "fileinput",
                "Upload csv trascript"
            }
            input {
                class: "border border-black rounded-md",
                type: "file",
                id: "fileinput",
                accept: "text/csv",
                onchange: move |evt| async move {
                    let Some(file_engine) = evt.files() else {return;};
                    let Some(file) = file_engine.files().into_iter().next() else { return;};
                    let Some(content) = file_engine.read_file_to_string(&file).await else {return;};
                    original_csv.set(content)
                }
            }
        }
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
                                onclick: select(id),
                                input { r#type: "checkbox", checked: selected }
                                SingleTransactionView { ..t.clone() }
                            }
                        }
                    })
            }
        }
    }
}
