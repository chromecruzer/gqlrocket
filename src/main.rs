#[macro_use] extern crate rocket;

use rocket::{State, routes};
use rocket::response::content::RawHtml;
use async_graphql::{Schema, EmptySubscription};
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use crate::schema::typedefs::create_schema;

mod schema;
mod resolvers;

#[get("/")]
fn graphql_playground() -> RawHtml<String> {
    let playground_html = playground_source(GraphQLPlaygroundConfig::new("/graphql"));
    RawHtml(playground_html)
}

#[post("/graphql", data = "<request>")]
async fn graphql_handler(
    schema: &State<Schema<resolvers::QueryRoot, resolvers::MutationRoot, EmptySubscription>>,
    request: GraphQLRequest
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}

#[launch]
fn rocket() -> _ {
    let schema = create_schema();

    rocket::build()
        .manage(schema)
        .mount("/", routes![graphql_playground, graphql_handler])
}

