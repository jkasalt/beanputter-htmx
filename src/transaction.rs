use chrono::NaiveDate;
use dioxus::prelude::*;
use rust_decimal::Decimal;

use crate::csv::UbsTransactionRecord;

pub(crate) struct SingleTransaction {
    date: NaiveDate,
    commodity: String,
    amount: Decimal,
    payee: String,
    description: String,
}

impl From<UbsTransactionRecord> for SingleTransaction {
    fn from(value: UbsTransactionRecord) -> Self {
        let amount = value.credit.unwrap_or(Decimal::ZERO) - value.debit.unwrap_or(Decimal::ZERO);
        let commodity = value.currency;

        Self {
            amount,
            commodity,
            date: value.date,
            payee: value.payee,
            description: value.description,
        }
    }
}

#[derive(Props, PartialEq, Eq, Clone, Hash, Debug)]
pub(crate) struct SingleTransactionViewProps {
    pub(crate) id: usize,
    pub(crate) date: NaiveDate,
    pub(crate) commodity: String,
    pub(crate) amount: Decimal,
    pub(crate) payee: String,
    pub(crate) description: String,
}

impl SingleTransactionViewProps {
    pub(crate) fn from_model_with_id(model: SingleTransaction, id: usize) -> Self {
        Self {
            id,
            date: model.date,
            commodity: model.commodity,
            amount: model.amount,
            payee: model.payee,
            description: model.description,
        }
    }
}

#[component]
pub fn SingleTransactionView(props: SingleTransactionViewProps) -> Element {
    rsx! {
        div { class: "single-transaction",
            "{props.date}"
            "{props.commodity}"
            "{props.amount}"
            "{props.payee}"
            "{props.description}"
        }
    }
}
