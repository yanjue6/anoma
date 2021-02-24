use super::types::Intent;

#[tarpc::service]
pub trait World {
    /// Returns a greeting for name.
    async fn hello(intent: Intent) -> String;
}
