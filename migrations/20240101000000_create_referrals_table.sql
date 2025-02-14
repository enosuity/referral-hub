CREATE TABLE referrals (
    id UUID PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    referral_code VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_referrals_user_id ON referrals(user_id);
CREATE INDEX idx_referrals_referral_code ON referrals(referral_code); 