use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct SearchQuery {
    search: Option<String>,
}
