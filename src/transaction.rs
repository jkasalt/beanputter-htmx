use chrono::NaiveDate;
use dioxus::prelude::*;
use rust_decimal::Decimal;

use crate::{csv::UbsTransactionRecord, AllTransactionsContext, Id};

pub(crate) struct Transaction {
    date: NaiveDate,
    commodity: String,
    amount: Decimal,
    payee: String,
    description: String,
}

impl From<UbsTransactionRecord> for Transaction {
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

#[component]
pub fn TransactionView(id: Id) -> Element {
    let all_transactions = use_context::<AllTransactionsContext>();
    let props = &all_transactions.0.read()[&id];
    rsx! {
        span { "{props.date}" }
        span { "{props.commodity}" }
        span { "{props.amount}" }
        span { "{props.payee}" }
        span { "{props.description}" }
    }
}
