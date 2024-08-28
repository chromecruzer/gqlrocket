pub mod query;    // Expose the query module
pub mod mutation; // Expose the mutation module

// Re-export the QueryRoot and MutationRoot
pub use query::QueryRoot;
pub use mutation::MutationRoot;
