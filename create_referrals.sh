#!/bin/bash

# Array of test data
declare -a referrals=(
    '{"user_id": "john_doe", "referral_code": "FRIEND01"}'
    '{"user_id": "jane_smith", "referral_code": "FAMILY02"}'
    '{"user_id": "bob_wilson", "referral_code": "TEAM03"}'
    '{"user_id": "alice_brown", "referral_code": "GROUP04"}'
    '{"user_id": "charlie_davis", "referral_code": "SHARE05"}'
)

# Create each referral
for data in "${referrals[@]}"; do
    echo "Creating referral: $data"
    curl -X POST http://localhost:8080/api/referrals \
        -H "Content-Type: application/json" \
        -d "$data"
    echo -e "\n"
done

# Get all referrals
echo "Getting all referrals:"
curl http://localhost:8080/api/referrals
echo -e "\n" 