#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationType {
    Inspires,
    Contradicts,
    Extends,
    DreamedIn,
    ForgottenBy,
    MergesWith,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: uuid::Uuid,
    pub to: uuid::Uuid,
    pub relation: RelationType,
    pub weight: f32,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}
