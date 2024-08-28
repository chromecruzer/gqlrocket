use async_graphql::{Context, Object};
use crate::schema::typedefs::User;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        "Hello, world!"
    }

    async fn user(&self, _ctx: &Context<'_>, id: i32) -> User {
        User { id, name: "Alice".to_string() }
    }
}
