use core::str;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use axum::{
    Form, Router,
    extract::{MatchedPath, Multipart, Request},
    response::IntoResponse,
    routing::{get, post},
};
use chrono::{DateTime, NaiveDate, Utc};
use maud::{DOCTYPE, Markup, html};
use rust_decimal::{Decimal, prelude::FromPrimitive};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

mod csv;

#[derive(Serialize, Deserialize, Hash, Debug, PartialEq, Eq)]
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

struct TransactionView {
    view_id: Uuid,
    payee: String,
    amount: MoneyAmount,
    date: NaiveDate,
}

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

fn main_view() -> MainView {
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

async fn load_file(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_owned();
        let content_type = field.content_type().unwrap().to_owned();
        let data = field.text().await.unwrap();

        info!("(`{name}`, type=`{content_type}`): `{data}`");
    }
    html! {
        h1 { "Loaded!" }
    }
}

fn upload_file_form() -> Markup {
    html! {
        form id="load-file" hx-encoding="multipart/form-data" hx-post="/load" {

          input type="file" name="file" {}
          button { "Upload" }
        }
    }
}

async fn index() -> Markup {
    html! {
        (DOCTYPE)
        html {
            (header())
            (body())
        }
    }
}

fn body() -> Markup {
    let view = main_view();
    html! {
        body {
            (upload_file_form())
            (grouper_form(&view))
        }
    }
}

fn header() -> Markup {
    // <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
    html! {
        head {
            meta charset="utf8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            script defer src="https://cdn.jsdelivr.net/npm/alpinejs@3.x.x/dist/cdn.min.js" {}
            script src="https://cdn.jsdelivr.net/npm/htmx.org@2.0.5/dist/htmx.min.js" {}
        }
    }
}

fn grouper_form(view: &MainView) -> Markup {
    html! {
        div x-data="{ selected: [] }" {
            p x-text="selected" {}
            form action="/group-up" method="post" {
                @for transaction_view in view.leftovers() {
                    li class="flex" {
                        (grouper_form_item(transaction_view))
                    }
                }
            }
        }
    }
}

fn grouper_form_item(tv: &TransactionView) -> Markup {
    html! {
        input
          type="checkbox"
          value=(tv.view_id)
          x-model="selected";

        div .flex.flex-col {
            p {(tv.payee)}
            p {(tv.amount)}
        }
        p {(tv.date)}
    }
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
        .route("/", get(index))
        .route("/group-up", post(group_up))
        .route("/load", post(load_file))
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
