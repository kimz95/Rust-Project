#[derive{Debug, FromRow, Deserialize, Serialize}]
pub struct game_model{
    pub id: Uuid,
    pub field_name: String,
    pub address: String,
    pub day: String,
}