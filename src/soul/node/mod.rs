use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Uuid,
    pub label: String,
    pub data: String, // contenu du souffle, idée, souvenir, etc.
    pub emotion: Option<String>, // ex: "inquiet", "fasciné"
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
