use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Referral {
    pub id: Uuid,
    pub user_id: String,
    pub referral_code: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReferralRequest {
    pub user_id: String,
    pub referral_code: String,
}
