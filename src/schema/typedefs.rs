use async_graphql::{Schema, EmptySubscription};
use crate::resolvers::{QueryRoot, MutationRoot};

// Define the `User` type here for shared access
#[derive(Default)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[async_graphql::Object]
impl User {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }
}

pub fn create_schema() -> Schema<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot::default(), MutationRoot::default(), EmptySubscription)
        .finish()
}
