#[cfg(test)]
mod tests {
    use crate::models::CreateReferralRequest;
    use crate::repository::ReferralRepository;
    use sqlx::PgPool;

    async fn setup() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/referral_db".to_string());
        
        PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to database")
    }

    #[actix_web::test]
    async fn test_create_referral() {
        let pool = setup().await;
        let repo = ReferralRepository::new(&pool);

        let request = CreateReferralRequest {
            user_id: "test_user".to_string(),
            referral_code: "TEST123".to_string(),
        };

        let result = repo
            .create_referral(&request.user_id, &request.referral_code)
            .await;

        assert!(result.is_ok());
        let referral = result.unwrap();
        assert_eq!(referral.user_id, request.user_id);
        assert_eq!(referral.referral_code, request.referral_code);
    }
} 