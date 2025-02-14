use sqlx::PgPool;
use uuid::Uuid;
use crate::{models::Referral, error::AppError};

pub struct ReferralRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> ReferralRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_referral(
        &self,
        user_id: &str,
        referral_code: &str,
    ) -> Result<Referral, AppError> {
        sqlx::query_as!(
            Referral,
            r#"
            INSERT INTO referrals (id, user_id, referral_code, created_at)
            VALUES ($1, $2, $3, NOW())
            RETURNING id, user_id, referral_code, created_at
            "#,
            Uuid::new_v4(),
            user_id,
            referral_code,
        )
        .fetch_one(self.pool)
        .await
        .map_err(AppError::DatabaseError)
    }

    pub async fn count_user_referrals(&self, user_id: &str) -> Result<i64, AppError> {
        sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM referrals
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_one(self.pool)
        .await
        .map(|row| row.count.unwrap_or(0))
        .map_err(AppError::DatabaseError)
    }

    pub async fn check_referral_code_exists(&self, referral_code: &str) -> Result<bool, AppError> {
        sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM referrals WHERE referral_code = $1
            ) as exists
            "#,
            referral_code
        )
        .fetch_one(self.pool)
        .await
        .map(|row| row.exists.unwrap_or(false))
        .map_err(AppError::DatabaseError)
    }

    pub async fn get_referrals(&self) -> Result<Vec<Referral>, AppError> {
        sqlx::query_as!(
            Referral,
            r#"
            SELECT id, user_id, referral_code, created_at
            FROM referrals
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(self.pool)
        .await
        .map_err(AppError::DatabaseError)
    }
}