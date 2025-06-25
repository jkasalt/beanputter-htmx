use std::fmt::Display;

use axum::{Router, response::IntoResponse, routing::get};
use chrono::{DateTime, NaiveDate, Utc};
use maud::{html, Markup, Render, DOCTYPE};
use rust_decimal::{prelude::FromPrimitive, Decimal};

mod csv_reader;

#[derive(Debug)]
enum Currency {
    Chf,
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            Self::Chf => "CHF",
        };
        write!(f, "{to_write}")
    }
}

#[derive(Debug)]
struct MoneyAmount {
    currency: Currency,
    amount: Decimal,
}

impl Display for MoneyAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

struct Transaction {
    payee: String,
    amount: MoneyAmount,
    date: DateTime<Utc>,
}

impl Transaction {
    fn new(payee: impl Into<String>, amount: f64) -> Self {
        let payee = payee.into();
        let amount = MoneyAmount {
            currency: Currency::Chf,
            amount: Decimal::from_f64(amount).unwrap(),
        };

        Self {
            payee, amount, date: Utc::now(),
        }
    }

    fn into_view(self) -> TransactionView {
        self.into()
    }
}

impl From<Transaction> for TransactionView {
    fn from(value: Transaction) -> Self {
        Self {
            date: value.date.date_naive(),
            payee: value.payee,
            amount: value.amount
        }
    }
}

struct TransactionView {
    payee: String,
    amount: MoneyAmount,
    date: NaiveDate,
}

impl Render for TransactionView {
    fn render(&self) -> Markup {
        html!{
            div .flex.flex-col {
                p { (self.payee) }
                p { (self.amount) }
            }
            p { (self.date) }
        }
    }
}

async fn transaction_card() -> impl IntoResponse {
    let transactions = vec![
        Transaction::new("Robert", 23.44),
        Transaction::new("Jenna Malabonga", -500.0),
        Transaction::new("Mourinho", 1.23),
    ];
    html! {
        (header())
        div flex.flex-col.space-y-4 {
            @for transaction in transactions {
                div .flex.space-x-8.outline-black {
                    input type="checkbox";
                    (transaction.into_view())
                }
            }
        }
    }
}

fn header() -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1.0";
        script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(transaction_card));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
