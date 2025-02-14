use actix_web::{get, post, web, HttpResponse};
use sqlx::PgPool;
use serde_json::json;

use crate::models::{CreateReferralRequest, Referral};
use crate::repository::ReferralRepository;
use crate::cache::CacheRepository;

const REFERRALS_CACHE_KEY: &str = "all_referrals";
const CACHE_EXPIRY_SECS: u64 = 300; // 5 minutes

#[post("/api/referrals")]
pub async fn create_referral(
    pool: web::Data<PgPool>,
    cache: web::Data<CacheRepository>,
    request: web::Json<CreateReferralRequest>,
) -> HttpResponse {
    let repo = ReferralRepository::new(pool.get_ref());
    
    // Check if user has reached the limit
    match repo.count_user_referrals(&request.user_id).await {
        Ok(count) if count >= 10 => {
            return HttpResponse::BadRequest()
                .json(json!({
                    "error": "User has reached maximum referral limit"
                }));
        }
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(json!({
                    "error": "Failed to check referral count"
                }));
        }
        _ => {}
    }

    // Check if referral code already exists
    match repo.check_referral_code_exists(&request.referral_code).await {
        Ok(true) => {
            return HttpResponse::BadRequest()
                .json(json!({
                    "error": "Referral code already exists"
                }));
        }
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(json!({
                    "error": "Failed to check referral code"
                }));
        }
        _ => {}
    }

    // Create new referral
    match repo.create_referral(&request.user_id, &request.referral_code).await {
        Ok(referral) => {
            // Invalidate the cache after creating new referral
            if let Err(e) = cache.set_key(REFERRALS_CACHE_KEY, "", Some(0)).await {
                log::warn!("Failed to invalidate cache: {}", e);
            }
            HttpResponse::Created().json(referral)
        }
        Err(_) => HttpResponse::InternalServerError()
            .json(json!({
                "error": "Failed to create referral"
            })),
    }
}

#[get("/api/referrals")]
pub async fn get_referrals(
    pool: web::Data<PgPool>,
    cache: web::Data<CacheRepository>,
) -> HttpResponse {
    // Try to get from cache first
    if let Ok(Some(cached_data)) = cache.get_key(REFERRALS_CACHE_KEY).await {
        if !cached_data.is_empty() {
            if let Ok(referrals) = serde_json::from_str::<Vec<Referral>>(&cached_data) {
                return HttpResponse::Ok().json(referrals);
            }
        }
    }

    // If not in cache or cache error, get from database
    let repo = ReferralRepository::new(pool.get_ref());
    match repo.get_referrals().await {
        Ok(referrals) => {
            // Cache the results
            if let Ok(json_data) = serde_json::to_string(&referrals) {
                if let Err(e) = cache.set_key(REFERRALS_CACHE_KEY, &json_data, Some(CACHE_EXPIRY_SECS)).await {
                    log::warn!("Failed to cache referrals: {}", e);
                }
            }
            HttpResponse::Ok().json(referrals)
        }
        Err(_) => HttpResponse::InternalServerError()
            .json(json!({
                "error": "Failed to fetch referrals"
            })),
    }
}
