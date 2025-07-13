use std::collections::HashMap;

use dioxus::prelude::*;
use transaction::{Transaction, TransactionView};

mod csv;
mod transaction;

static TAILWIND: Asset = asset!("/assets/tailwind.css");

#[derive(Clone)]
struct AllTransactionsContext(pub(crate) Signal<HashMap<Id, Transaction>>);

#[derive(Clone)]
struct SelectionContext(pub(crate) Signal<HashMap<Id, bool>>);

impl SelectionContext {
    fn has(&self, id: &Id) -> bool {
        *self.0.read().get(id).unwrap_or(&false)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Id(usize);

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

    let all_transactions = use_context_provider(move || {
        AllTransactionsContext(Signal::new(
            crate::csv::read_ubs_csv(&original_csv.read())
                .unwrap()
                .into_iter()
                .map(Transaction::from)
                .enumerate()
                .map(|(i, t)| (Id(i), t))
                .collect(),
        ))
    });

    let mut leftovers = use_signal(|| {
        all_transactions
            .0
            .read()
            .keys()
            .copied()
            .collect::<Vec<_>>()
    });

    let mut groups = use_signal(Vec::new);

    let mut selection = use_context_provider(|| {
        SelectionContext(Signal::new(
            leftovers
                .read()
                .iter()
                .map(|id| (*id, false))
                .collect::<HashMap<_, _>>(),
        ))
    });

    let group_up = || {
        move |_| {
            let new_group: Vec<Id> = leftovers
                .write()
                .extract_if(.., |id| selection.has(id))
                .collect();
            for id in &new_group {
                selection
                    .0
                    .write()
                    .entry(*id)
                    .and_modify(|v| *v = false)
                    .or_insert(false);
            }
            groups.write().push(new_group);
        }
    };

    rsx! {
        document::Stylesheet { href: TAILWIND }
        div { class: "flex flex-row gap-2",
            label { r#for: "fileinput", "Upload csv trascript" }
            input {
                class: "file:border file:border-black file:rounded",
                r#type: "file",
                id: "fileinput",
                accept: "text/csv",
                onchange: move |evt| async move {
                    let Some(file_engine) = evt.files() else {
                        return;
                    };
                    let Some(file) = file_engine.files().into_iter().next() else {
                        return;
                    };
                    let Some(content) = file_engine.read_file_to_string(&file).await else {
                        return;
                    };
                    original_csv.set(content)
                },
            }
            button {
                class: "border rounded border-black cursor-pointer",
                onclick: group_up(),
                "group up"
            }
        }
        Groups { groups }
        Leftovers { ids: leftovers }
    }
}

#[component]
fn Groups(groups: ReadOnlySignal<Vec<Vec<Id>>>) -> Element {
    rsx! {
        ul {
            {groups.read().iter().map(|group| rsx! {
                li {
                    ul {
                        {group.iter().map(|&id| rsx! {
                            li { class: "bg-blue-100",
                                TransactionView { id }
                            }
                        })}
                    }
                }
            })}
        }
    }
}

#[component]
fn Leftovers(ids: ReadOnlySignal<Vec<Id>>) -> Element {
    rsx! {
        ul {
            {ids.read().iter().map(|id| rsx! {
                li {
                    Leftover { id: *id }
                }
            })}
        }
    }
}

#[component]
fn Leftover(id: Id) -> Element {
    let mut selection = use_context::<SelectionContext>();
    let toggle_select = |id| {
        move |_| {
            selection
                .0
                .write()
                .entry(id)
                .and_modify(|v| *v = !*v)
                .or_insert(true);
        }
    };
    let is_selected = selection.has(&id);
    let common_class = "flex flex-row cursor-pointer gap-2";
    let selected_class = if is_selected {
        "bg-green-100"
    } else {
        "bg-gray-200"
    };
    let class = format!("{common_class} {selected_class}");
    rsx! {
        button { class, onclick: toggle_select(id),
            input { r#type: "checkbox", checked: is_selected }
            TransactionView { id }
        }
    }
}
