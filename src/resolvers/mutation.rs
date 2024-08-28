use async_graphql::{Context, Object};
use crate::schema::typedefs::User;

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, _ctx: &Context<'_>, name: String) -> User {
        User { id: 1, name }
    }
}
