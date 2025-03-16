use axum::{
    http::StatusCode,
    response::{IntoResponse, Html},
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use async_graphql::{EmptySubscription, http::GraphiQLSource, Object, Schema};
use async_graphql_axum::GraphQL;
use futures::lock::Mutex;
use slab::Slab;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // REST API用
    // // build our application with a route
    // let app = Router::new()
    //     // `GET /` goes to `root`
    //     .route("/", get(root))
    //     // `POST /users` goes to `create_user`
    //     .route("/cards", get(fetch_card))
    //     .route("/cards", post(create_card))
    //     .route("/cards/{id}", post(update_card))
    //     .route("/cards/{id}", delete(delete_card));

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Storage::default())
        .finish();

    let app = Router::new()
        .route(
            "/",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        );

    println!("GraphiQL IDE: http://localhost:8000");

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Serialize, Clone)]
pub struct Card {
    id: String,
    value: i64,
}

#[Object]
impl Card {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn value(&self) -> &i64 {
        &self.value
    }
}

async fn fetch_card() -> impl IntoResponse {
    let card = Card {
        id: Uuid::new_v4().to_string(),
        value: 0,
    };
    (StatusCode::OK, Json(card))
}

async fn create_card(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateCard>,
) -> impl IntoResponse {
    // insert your application logic here
    let card = Card {
        id: Uuid::new_v4().to_string(),
        value: payload.value,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(card))
}

async fn update_card(
    Json(payload): Json<UpdateCard>,
) -> impl IntoResponse {
    let card = Card {
        id: payload.id.to_string(),
        value: payload.value,
    };
    (StatusCode::OK, Json(card))
}

async fn delete_card(
    Json(payload): Json<DeleteCard>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(payload.id))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateCard {
    value: i64,
}

#[derive(Deserialize)]
struct UpdateCard {
    id: Uuid,
    value: i64,
}

#[derive(Deserialize)]
struct DeleteCard {
    id: Uuid,
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .finish(),
    )
}

pub struct QueryRoot;
pub struct MutationRoot;
pub struct SubscriptionRoot;
pub type CardsSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub type Storage = Arc<Mutex<Slab<Card>>>;


#[Object]
impl QueryRoot {
    async fn card(&self) -> Card {
        let card = Card {
            id: Uuid::new_v4().to_string(),
            value: 0,
        };

        card
    }
}

#[Object]
impl MutationRoot {
    async fn create_card(&self, value: i64) -> Card {
        let card = Card {
            id: Uuid::new_v4().to_string(),
            value: value,
        };

        card
    }

    async fn update_card(&self, id: String, value: i64) -> Card {
        let card = Card {
            id: id,
            value: value,
        };

        card
    }

    async fn delete_card(&self, id: String) -> String {
        id
    }
}
