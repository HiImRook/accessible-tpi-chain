use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BroadcastAnchor {
    pub anchor_id: String,
    pub slot: u64,
    pub message_type: String,
    pub emitted_at: u64,
    pub emitter_peer_id: String,
    pub block_hash: Option<String>,
    pub broadcast_scope: String,
    pub reference_points: Vec<String>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PeerResponseSample {
    pub sample_id: String,
    pub anchor_id: String,
    pub peer_id: String,
    pub observed_at: u64,
    pub delta_ms: u64,
    pub response_type: String,
    pub message_kind: String,
    pub transport_addr: String,
    pub session_id: String,
    pub rtt_ms: Option<u64>,
    pub rtt_asymmetry_ms: Option<i64>,
    pub hop_signature: Option<String>,
    pub validity_flags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RttLocationProfile {
    pub peer_id: String,
    pub reference_point_id: String,
    pub sample_count: u64,
    pub rtt_samples_ms: Vec<u64>,
    pub rtt_min_ms: u64,
    pub rtt_median_ms: u64,
    pub rtt_mean_ms: f64,
    pub rtt_stddev_ms: f64,
    pub rtt_jitter_score: f64,
    pub rtt_asymmetry_ms: f64,
    pub baseline_distance_score: f64,
    pub physics_floor_violation: bool,
    pub location_confidence_score: f64,
    pub infrastructure_similarity_score: f64,
    pub vpn_proxy_likelihood: f64,
    pub datacenter_likelihood: f64,
    pub residential_likelihood: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CorrelationWindow {
    pub window_id: String,
    pub subject_id: String,
    pub window_start: u64,
    pub window_end: u64,
    pub observer_period_start: u64,
    pub observer_period_end: u64,
    pub sample_count: u64,
    pub subwindow_span_secs: u64,
    pub subwindow_count: u64,
    pub broadcast_anchor_ids: Vec<String>,
    pub peer_sample_ids: Vec<String>,
    pub arrival_variance_ms: f64,
    pub arrival_stddev_ms: f64,
    pub heartbeat_sync_score: f64,
    pub reaction_correlation_score: f64,
    pub rtt_profile_score: f64,
    pub rtt_asymmetry_score: f64,
    pub location_confidence_score: f64,
    pub cluster_id: Option<String>,
    pub cluster_size: u64,
    pub confidence_trend: String,
    pub flag_events: Vec<String>,
    pub decay_rate_applied: f64,
    pub analysis_notes: Option<String>,
}

pub fn record_broadcast_anchor() -> BroadcastAnchor {
    unimplemented!()
}

pub fn record_peer_response_sample() -> PeerResponseSample {
    unimplemented!()
}

pub fn build_correlation_window() -> CorrelationWindow {
    unimplemented!()
}
