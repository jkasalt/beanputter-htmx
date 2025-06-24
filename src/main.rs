use std::fmt::Display;

use axum::{Router, response::IntoResponse, routing::get};
use chrono::{DateTime, Utc};
use maud::{Markup, html};
use rust_decimal::Decimal;

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

async fn transaction_card(
    Transaction {
        payee,
        amount,
        date,
    }: Transaction,
) -> impl IntoResponse {
    html! {
        ul {
            li { (payee) }
            li { (amount) }
            li { (date) }
        }
    }
}

#[tokio::main]
async fn main() {
    let transaction = Transaction {
        payee: "Robert".to_string(),
        amount: MoneyAmount {
            currency: Currency::Chf,
            amount: Decimal::new(2234, 5),
        },
        date: Utc::now(),
    };
    let app = Router::new().route("/", get(transaction_card(transaction)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
