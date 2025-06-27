use core::str;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use askama::Template;
use axum::{
    Form, Router,
    extract::{MatchedPath, RawForm, Request},
    response::IntoResponse,
    routing::{get, post},
};
use chrono::{DateTime, NaiveDate, Utc};
use either::Either;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

mod csv_reader;

#[derive(Serialize, Hash, Debug)]
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

#[derive(Serialize, Hash, Debug)]
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
            view_id: Uuid::new_v4(),
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
#[derive(Template, Hash)]
#[template(ext = "html", in_doc = true)]
struct TransactionView {
    view_id: Uuid,
    payee: String,
    amount: MoneyAmount,
    date: NaiveDate,
}

#[derive(Template, askama_web::WebTemplate)]
#[template(path = "index.html")]
struct MainView {
    all: HashMap<usize, TransactionView>,
    grouped: Vec<Vec<usize>>,
    leftover: HashSet<usize>,
}

impl MainView {
    fn new(transactions: Vec<TransactionView>) -> Self {
        let all: HashMap<usize, TransactionView> = transactions.into_iter().enumerate().collect();
        let leftover = all.keys().copied().collect();

        Self {
            all,
            leftover,
            grouped: Vec::new(),
        }
    }

    fn leftovers(&self) -> impl Iterator<Item = &TransactionView> {
        self.all
            .iter()
            .filter_map(|(k, v)| self.leftover.contains(k).then_some(v))
    }
}

async fn main_view() -> impl IntoResponse {
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

#[derive(Debug, Deserialize)]
struct GroupRequest {
    selected: String,
}

async fn group_up(Form(req): Form<GroupRequest>) -> impl IntoResponse {
    // let group = form
    //     .0
    //     .split(|c| *c == b'&')
    //     .map(|s| s.split(|c| *c == b'\'').nth(1).unwrap())
    //     .map(|s| str::from_utf8(s).unwrap())
    //     .map(|s| s.parse::<usize>().unwrap())
    //     .collect::<Vec<_>>();

    tracing::info!("{req:?}");

    ""
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(main_view))
        .route("/group-up", post(group_up))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
