use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use askama::Template;
use axum::{Router, response::IntoResponse, routing::get};
use chrono::{DateTime, NaiveDate, Utc};
use either::Either;
use rust_decimal::{Decimal, prelude::FromPrimitive};

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
            payee,
            amount,
            date: Utc::now(),
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
            amount: value.amount,
        }
    }
}


/// ```askama
///     <div class="flex flex-col">
///         <p>{{payee}}</p>
///         <p>{{amount}}</p>
///     </div>
///     <p>{{date}}</p>
/// ```
///
#[derive(Template)]
#[template(
    ext = "html",
    in_doc = true,
)]
struct TransactionView {
    payee: String,
    amount: MoneyAmount,
    date: NaiveDate,
}

#[derive(Template, askama_web::WebTemplate)]
#[template(path = "index.html")]
struct MainView {
    all: HashMap<usize, TransactionView>,
    grouped: HashMap<usize, Vec<usize>>,
    leftover: HashSet<usize>,
}

impl MainView {
    fn new(transactions: Vec<TransactionView>) -> Self {
        let all: HashMap<usize, TransactionView> = transactions.into_iter().enumerate().collect();
        let leftover = all.keys().copied().collect();

        Self {
            all,
            leftover,
            grouped: HashMap::new(),
        }
    }

    fn leftovers(&self) -> impl Iterator<Item = (&usize, &TransactionView)> {
        self.all
            .iter()
            .filter(|(k, v)| self.leftover.contains(k))
    }

    fn group(&self, k: usize) -> impl Iterator<Item = &TransactionView> {
        match self.grouped.get(&k) {
            None => Either::Left(std::iter::empty()),
            Some(group) => Either::Right(group.iter().flat_map(|i| self.all.get(i))),
        }
    }
}

async fn transaction_card() -> impl IntoResponse {
    let transactions = vec![
        Transaction::new("Robert", 23.44),
        Transaction::new("Jenna Malabonga", -500.0),
        Transaction::new("Mourinho", 1.23),
    ]
    .into_iter()
    .map(|t| t.into_view())
    .collect();

    MainView::new(transactions)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(transaction_card));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
