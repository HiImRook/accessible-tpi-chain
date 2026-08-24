use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlagRecord {
    pub flag_id: String,
    pub timestamp: u64,
    pub confidence_at_flag: f64,
    pub reason_codes: Vec<String>,
    pub cluster_id: Option<String>,
    pub resolved: bool,
    pub resolution_timestamp: Option<u64>,
    pub resolution_outcome: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NodeIntegrityScore {
    pub subject_id: String,
    pub current_confidence: f64,
    pub confidence_floor: f64,
    pub confidence_ceiling: f64,
    pub network_timing_component: f64,
    pub rtt_component: f64,
    pub location_component: f64,
    pub behavioral_component: f64,
    pub wallet_component: f64,
    pub cluster_component: f64,
    pub promotion_blocking_threshold: f64,
    pub promotion_ready_threshold: f64,
    pub last_updated_at: u64,
    pub resolution_state: String,
    pub recovery_eligibility: bool,
    pub historical_flags: Vec<FlagRecord>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FlagPattern {
    pub subject_id: String,
    pub total_flag_count: u64,
    pub flag_timestamps: Vec<u64>,
    pub flag_intervals_secs: Vec<u64>,
    pub mean_interval_secs: f64,
    pub interval_stddev_secs: f64,
    pub cluster_ids_seen: Vec<String>,
    pub repeated_cluster_count: u64,
    pub peak_confidence_reached: f64,
    pub flags_post_recovery: u64,
    pub pattern_class: String,
    pub cross_node_matches: Vec<String>,
    pub last_analyzed_at: u64,
}

pub fn aggregate_node_integrity_score() -> NodeIntegrityScore {
    unimplemented!()
}

pub fn decide_promotion() -> bool {
    unimplemented!()
}

pub fn analyze_flag_pattern() -> FlagPattern {
    unimplemented!()
}
