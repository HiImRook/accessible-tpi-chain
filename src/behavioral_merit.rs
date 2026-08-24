use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OperationalIdentity {
    pub node_id: String,
    pub wallet_address: String,
    pub wallet_public_key: String,
    pub first_seen_at: u64,
    pub last_seen_at: u64,
    pub wallet_age_days: u64,
    pub last_signed_at: Option<u64>,
    pub signing_cadence_score: f64,
    pub transaction_activity_score: f64,
    pub operator_behavior_score: f64,
    pub identity_stability_score: f64,
}

pub fn update_operational_identity() -> OperationalIdentity {
    unimplemented!()
}

pub fn compute_behavioral_merit_score() -> f64 {
    unimplemented!()
}
