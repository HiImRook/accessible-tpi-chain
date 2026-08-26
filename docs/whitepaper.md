# Valid Blockchain: A Merit-Based, Zero Footprint Layer 1 with Three-Party Integrity Consensus

**Author:** Michael "Rook" Carter
**White Paper Version:** 1.0
**Status:** Pre-release beta v0.8.0
**Date:** August 26th, 2026
**Website** https://hiimrook.github.io/accessible-tpi-chain/
**Repository:** https://github.com/HiImRook/accessible-tpi-chain
**Discord:** https://discord.gg/2SP383cJs9

---

## Overview

Valid Blockchain is a sovereign Layer 1 blockchain written from scratch in Rust, built around Three-Party Integrity (TPI) - an original consensus mechanism that does not derive from any existing model. Valid replaces capital-based and compute-based Sybil resistance with a merit system denominated in time and demonstrated behavior, making validator participation accessible to anyone with modest hardware and an internet connection.

**TPI (Three-Party Integrity)** is the consensus mechanism at the core of Valid Blockchain. Rather than requiring large committees, expensive hardware, or token stakes, TPI selects exactly three validators per block slot, has each independently compute a candidate block hash, and resolves agreement through merit-based producer selection. The result is sub-second finality on 10-second blocks with no capital requirement.

**Zero Footprint** is the architectural philosophy governing the network layer. Raw IP addresses are never stored as peer identity. Peer identity is derived, hashed, and rotated. Certificates are generated in memory and never persisted. Nothing is retained that does not have a specific operational purpose. You cannot leak what you never kept.

The network enforces these principles at every layer: identity, transport, storage, and publication. This paper describes the TPI consensus mechanism, the merit accumulation and decay model, the two-tier observer and validator network architecture, the heartbeat correlation detection system for Sybil resistance, the Zero Footprint network layer, the archive and persistence model, and the VLid token economics.

---

## 1. The Problem with Modern Blockchains

The dominant blockchain architectures of the current generation impose participation barriers that contradict the decentralization they claim to provide.

Proof of Work requires specialized hardware and sustained electricity expenditure that concentrates validator power in regions with cheap industrial electricity. The result is that Bitcoin mining is effectively controlled by a small number of large mining pools operating industrial-scale facilities. A person with a laptop cannot meaningfully participate.

Proof of Stake requires capital at stake, concentrating validator power among those who already hold significant token balances. Ethereum requires 32 ETH to run a validator. At any meaningful ETH price, that is a capital barrier that excludes the majority of people on Earth. Those who can afford to stake more have proportionally more influence over the network they claim is decentralized.

Both models treat the cost of participation as the security mechanism. This is a category error. Cost-as-security is access control, not cryptographic security. A sufficiently capitalized adversary bypasses both models by simply purchasing the required hashrate or stake. The security threshold is denominated in a liquid asset and can be crossed by anyone with enough money.

The geographic consequences are significant and rarely discussed. Ethereum validators are concentrated in the United States, the UK, Germany, and a handful of other wealthy countries. Bitcoin mining is similarly concentrated. A validator in Nigeria, The Philippines, or India with a 4GB Raspberry Pi and a residential internet connection is explicitly excluded from meaningful participation in both networks. This is not an edge case or an acceptable tradeoff. It is a structural failure of decentralization dressed up as a feature.

**Chain state bloat** is a third problem that compounds over time. Bitcoin's full node requires hundreds of gigabytes of storage. Ethereum's state has grown to the point where running a full node requires significant hardware. Both networks have struggled to address this without either centralizing validation or compromising historical integrity. The problem has generally been treated as unsolvable: the chain grows, the requirements grow, and participation narrows.

Valid Blockchain was designed to address all three failures directly. Every architectural decision was made with one question in mind: can someone in a developing region with modest hardware participate as a first-class validator, indefinitely, without the hardware requirements growing over time?

The answer is yes. This has always been the primary goal.

---

## 2. Why Rust

Valid Blockchain is written entirely in Rust. This is not an aesthetic preference. It is an architectural requirement that flows directly from the project's goals.

**Memory safety without a garbage collector.** Rust's ownership and borrow checker system enforces memory safety at compile time. Buffer overflows, use-after-free errors, and null pointer dereferences - the class of vulnerabilities responsible for the majority of critical security bugs in C and C++ systems - cannot exist in safe Rust code. A blockchain node is a long-running networked process handling untrusted input from potentially adversarial peers. Memory safety is not optional.

**Performance without compromise.** Rust produces native machine code with performance comparable to C and C++. There is no garbage collector pause, JVM startup overhead, or interpreted execution. A Valid Blockchain node can run on a Raspberry Pi with 4GB of RAM while handling real-time block production and peer networking because the runtime overhead is minimal.

**Deterministic behavior.** Consensus requires that multiple validators produce identical results from identical inputs. Rust's deterministic execution model and explicit handling of randomness and system calls make it far easier to reason about and verify the correctness of consensus-critical code paths.

**Supply chain security.** Rust's Cargo ecosystem and the vendoring model described in Section 3 provide strong supply chain guarantees that are harder to achieve in ecosystems with less explicit dependency management.

The following example illustrates Rust's ownership model preventing a class of bugs that would be silent failures in C:

```rust
fn process_peer_message(peer: &mut PeerInfo, message: PeerMessage) {
    // Rust's borrow checker ensures peer cannot be used
    // elsewhere while this mutable reference exists.
    // No locking ceremony required for single-threaded paths.
    peer.last_seen = current_timestamp();
    peer.message_count += 1;
}
```

In a garbage-collected language, the equivalent code might silently share the peer reference across threads, producing race conditions. In Rust, the compiler rejects such code at compile time. The node cannot ship with this class of bug.

---

## 3. Vendored Dependencies: A Requirement, Not a Choice

All dependencies in Valid Blockchain are vendored. The vendor directory contains a complete, auditable snapshot of every crate the project depends on, committed directly to the repository. This is not a convenience feature, but rather an important security requirement.

**The supply chain attack problem.** Software supply chain attacks have become one of the most effective vectors for compromising widely-deployed systems. The SolarWinds attack, the event-stream npm incident, and dozens of smaller attacks all followed the same pattern: a trusted dependency was compromised, and every downstream consumer was affected automatically. A blockchain node that pulls dependencies from a remote registry at build time is one compromised crate away from shipping malicious code to every validator on the network.

**What vendoring provides.** When dependencies are vendored, the build is entirely reproducible from the repository contents alone. No network access is required. No registry needs to be trusted. No dependency can be silently updated between builds. The exact code that was reviewed and tested is the exact code that ships.

```toml
# .cargo/config.toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```

With this configuration, `cargo build --release` never contacts the internet. It builds exclusively from the vendor directory. An auditor reviewing the repository has complete visibility into every line of code that will execute in the final binary.

**Cargo audit integration.** The CI pipeline runs `cargo audit` on every commit against the RustSec advisory database. Any dependency with a known vulnerability causes the build to fail. This provides ongoing vulnerability monitoring against the vendored snapshot. When a vulnerability is identified, the affected crate is updated in vendor and committed explicitly, with the fix documented in the changelog.

This should not be mistaken for overhead. This is the minimum acceptable standard for software that manages financial assets on behalf of its users.

---

## 4. Three-Party Integrity Consensus

### 4.1 Overview and Bootstrap Merit

Three-Party Integrity (TPI) is an original consensus mechanism. It is not derived from Nakamoto consensus, Casper, PBFT, Tendermint, HotStuff, or any existing model. The three-party structure, the merit-based producer resolution, and the quarantine mechanic were designed here as a unified system.

The core insight behind TPI is that consensus does not require a large committee. It requires independent verification by a sufficient number of parties to make collusion detectable and costly. Three independent validators, each computing a candidate block hash from the same transaction set, produce a result that is either in agreement or in detectable disagreement. Agreement produces a block. Disagreement identifies a misbehaving validator and penalizes them through the merit system.

**Bootstrap merit from testnet.** A critical property of the Valid Blockchain launch model is that the initial validator set does not begin mainnet with zero merit history. The testnet period, described in detail in Section 10, serves as the merit bootstrapping phase. Validators who participate in testnet accumulate behavioral merit, block production merit, and leaderboard standing over the six-month testnet window. This accumulated merit carries forward to mainnet as the seed standing for each validator. The first mainnet block producers are therefore not operating from a blank slate. They are operating from a verified, multi-month behavioral record that the correlation detection system and the merit system both recognize as legitimate.

This is architecturally significant. A system where the first block producers have no established merit is vulnerable to a Sybil attack at genesis: an attacker who connects early and produces blocks aggressively can establish an artificial merit advantage before legitimate validators arrive. Bootstrap merit from testnet eliminates this window. Mainnet launch begins with a validator set whose standing has already been earned and verified.

### 4.2 Slot Structure

The Valid Blockchain operates on a 10-second slot cycle. Each slot has a fixed structure:

```
T+0ms to T+6000ms:     TPI window
T+6000ms to T+8000ms:  Buffer window
T+8000ms to T+10000ms: Racer window
T+10000ms:             Slot boundary, next slot begins
```

The genesis timestamp establishes the absolute time reference for all slot boundaries. Slot N begins at:

```
slot_start_ms = genesis_timestamp_ms + (N * 10000)
```

All validators compute slot boundaries independently from the shared genesis timestamp. No slot announcement or time negotiation occurs between validators.

### 4.3 Validator Selection

For each slot, exactly three validators are selected from the active validator pool using a deterministic, unpredictable selection function:

```
seed = SHA256(slot_number_as_little_endian_bytes)

for each validator_id in validator_pool:
    sort_key(validator_id) = SHA256(seed || validator_id_bytes)

selected = top_3_by_sort_key(validator_pool)
```

This selection function is deterministic, unpredictable ahead of time, stable across pool changes, and independently verifiable by any observer.

```rust
pub fn select_tpi_validators(slot: u64, validators: &[String]) -> Vec<String> {
    if validators.is_empty() {
        return Vec::new();
    }
    let mut hasher = Sha256::new();
    hasher.update(slot.to_le_bytes());
    let seed = hasher.finalize();
    let mut indices: Vec<usize> = (0..validators.len()).collect();
    indices.sort_by_key(|&i| {
        let mut h = Sha256::new();
        h.update(&seed);
        h.update(validators[i].as_bytes());
        h.finalize()
    });
    let selection_size = TPI_GROUP_SIZE.min(validators.len());
    indices.into_iter()
        .take(selection_size)
        .map(|i| validators[i].clone())
        .collect()
}
```

**Why more validators strengthens the network.** In most blockchain architectures, adding validators increases communication overhead and slows consensus. TPI does not have this property. Validator selection is a local computation performed independently by each node. It requires no inter-validator communication to determine who was selected. Adding more validators to the pool increases the unpredictability of selection, increases the cost of a Sybil attack (as described in Section 5), and improves geographic and infrastructure diversity, all without adding any communication rounds to the consensus process. A network with 1,000 validators is not slower than a network with 10 validators. **It is dramatically more secure.**

### 4.4 Block Hash Computation

Each selected validator independently constructs a candidate block from the current mempool state and computes a deterministic block hash:

```rust
pub fn compute_block_hash(block: &Block) -> String {
    let mut hasher = Sha256::new();
    hasher.update(block.slot.to_le_bytes());
    hasher.update(block.parent_hash.as_bytes());
    hasher.update(block.producer.as_bytes());
    hasher.update(block.timestamp.to_le_bytes());
    for tx in &block.transactions {
        hasher.update(tx.from.as_bytes());
        hasher.update(tx.from_pubkey.as_bytes());
        hasher.update(tx.to.as_bytes());
        hasher.update(tx.amount.to_le_bytes());
        hasher.update(tx.nonce.to_le_bytes());
        hasher.update(tx.fee.to_le_bytes());
        hasher.update(tx.signature.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}
```

Transactions are ordered by fee descending before hashing, ensuring that all validators with the same mempool state produce identical hashes for identical candidate blocks.

### 4.5 Consensus Resolution

```rust
pub fn check_tpi_consensus(responses: Vec<TpiHashMessage>) -> TpiConsensus {
    if responses.len() < 2 {
        return TpiConsensus::InsufficientData;
    }
    let mut hash_counts: HashMap<String, Vec<String>> = HashMap::new();
    for response in &responses {
        hash_counts
            .entry(response.block_hash.clone())
            .or_insert_with(Vec::new)
            .push(response.validator_id.clone());
    }
    if responses.len() == 3 {
        if hash_counts.len() == 1 {
            return TpiConsensus::Perfect(responses[0].block_hash.clone());
        }
        for (hash, validators) in hash_counts {
            if validators.len() >= 2 {
                let outlier = responses
                    .iter()
                    .find(|r| r.block_hash != hash)
                    .map(|r| r.validator_id.clone())
                    .unwrap_or_default();
                return TpiConsensus::TwoOfThree(hash, outlier);
            }
        }
        return TpiConsensus::NoConsensus;
    }
    TpiConsensus::InsufficientData
}
```

**Perfect consensus (3 of 3):** All three validators submitted the same hash. The highest-merit validator produces the block.

**Partial consensus (2 of 3):** Two agree, one does not. The outlier receives a quarantine and merit penalty. The highest-merit agreeing validator produces the block. *Future scope: the racer will be brought in as an independent verifier. If the racer's hash matches the producer, the block is confirmed. If the racer disagrees with the producer, the racer produces instead and the entire original trio is quarantined pending investigation, during which the racer's hash is used to re-evaluate whether the original outlier was actually correct.*

**No consensus:** All three disagree, or fewer than two responded. No block is produced by the selection in this slot. The racer activates and produces the block. In the chance the racer fails, **an empty block is produced on schedule to preserve slot timing. The chain never stalls..**

### 4.6 Merit-Based Producer Resolution

```rust
pub fn select_broadcaster_by_merit(validators: &[(String, u64)]) -> String {
    if validators.is_empty() {
        return String::new();
    }
    let mut sorted = validators.to_vec();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted[0].0.clone()
}
```

### 4.7 Racer Backup System

If no block is produced during the TPI and buffer windows, the racer activates. A backup validator is selected deterministically and produces a block unconditionally (with required merit and success ratio), preventing slot skipping under network partition or validator failure. Racer selection uses the same seeding mechanism as primary selection with a separate derivation path.

### 4.8 Finality

A block achieves finality the moment it is produced and broadcast. There is no probabilistic finality period and no fork resolution mechanism. Finality is sub-second within the 10-second slot window. TPI resolves disagreement before block production rather than after, eliminating the fork condition that requires probabilistic finality in Nakamoto-derived systems.

### 4.9 Anti-Frontrunning

Block producers use the parent block hash as a deterministic seed for transaction ordering within a block. This eliminates miner extractable value (MEV) by making transaction ordering unpredictable before the parent block is finalized, while remaining independently verifiable after the fact.

---

## 5. Merit System

### 5.1 Two Kinds of Merit: Validator Merit and Behavioral Merit

The merit system in Valid Blockchain operates on two distinct but complementary layers. Understanding the difference between them is important for understanding how the system achieves its security properties.

**Validator merit** is derived directly from participation in the TPI consensus process. It answers the question: how reliably and honestly has this node participated in block production? A validator that consistently produces correct blocks, agrees with consensus outcomes, and responds reliably during their selected slots accumulates validator merit. A validator that submits incorrect hashes, fails to respond when selected, or repeatedly triggers the racer accumulates penalties. Validator merit is the primary input to producer selection - when multiple validators agree on a hash, the one with the highest validator merit produces the block.

**Behavioral merit** captures a broader set of network and on-chain behaviors that validator merit alone cannot measure. It answers the question: does this node behave like a genuine, human-operated participant over time? Behavioral merit tracks wallet continuity, uptime patterns, transaction activity cadence, gossip integrity, and eventually participation in governance and ecosystem activities. Behavioral merit is the primary input to the promotion decision - a node can accumulate validator merit inside the observer pool, but promotion to the validator pool also requires demonstrated behavioral legitimacy over the 90-day observation period.

The two layers are complementary. Validator merit alone is gameable: an automated script can produce correct block hashes reliably without any human involvement. Behavioral merit is much harder to automate convincingly across a 90-day window, especially when the heartbeat correlation detection system described in Section 5.4 is actively distinguishing scripted behavior from organic behavior.

### 5.2 Block Production Merit

Validators earn merit by:

- Correctly computing and submitting a block hash during the TPI window
- Agreeing with the consensus outcome
- Successfully producing a block when selected as the highest-merit agreeing validator
- Serving as a reliable racer when called upon

Validators lose merit through:

- Submitting a hash that disagrees with the consensus outcome (quarantine trigger)
- Failing to respond during the TPI window when selected
- Repeated racer failures

Quarantine is a temporary state in which a validator's merit score is penalized significantly and their eligibility for TPI selection is suspended for a configurable number of slots. This is the protocol's mechanism for removing bad actors from the producer pool without permissioned exclusion.

### 5.3 Behavioral Merit

The behavioral merit module derives scores from the following observable signals, all verifiable from chain state or network telemetry without external dependencies:

**Uptime continuity:** Consistent network presence accumulates uptime merit. Extended disconnection periods cause uptime merit to decay.

**Wallet continuity:** A validator whose associated wallet address has been active on-chain for an extended period - receiving block rewards, submitting transactions - accumulates wallet age merit. A fresh wallet carries no wallet age merit.

**Transaction activity cadence:** Human-operated wallets show natural variance in transaction timing. Scripted wallets show regular, clock-like patterns detectable through statistical analysis.

**Correct gossip propagation:** Validators that consistently propagate valid transactions and blocks without injecting invalid gossip accumulate gossip integrity merit.

### 5.4 Merit Decay: Why It Is Necessary

Merit decay is not a punitive feature. It is the mechanism that makes the entire Sybil resistance model work.

Without decay, a validator could connect to the network, accumulate merit through a burst of activity over a few weeks, and then coast indefinitely on that stored standing while contributing nothing. Worse, an attacker could spin up thousands of nodes, run them aggressively for a short period to accumulate merit, and then have thousands of high-merit promoted validators sitting dormant waiting to execute an attack. The accumulated merit would be a weapon held in reserve.

With decay, merit erodes continuously without active participation. The decay rate is calibrated so that a single dedicated validator maintaining normal activity accumulates merit faster than it decays, maintaining a stable score above the promotion threshold under ordinary operation. A dormant node loses ground continuously. A Sybil farm of 1,000 nodes that cannot be individually maintained at the required behavioral level loses ground on all unmaintained nodes continuously. The attack collapses under its own maintenance burden.

This is the key insight: decay converts the attack from a one-time infrastructure cost into an ongoing, insurmountable operational burden. Each node in a Sybil farm requires independent, continuous, human-level behavioral attention to remain above the merit threshold. No attacker can provide that for thousands of nodes simultaneously. Decay makes this mathematically certain rather than merely difficult.

But decay alone is not sufficient. A sophisticated attacker could write scripts that produce jitter, and automatically maintain the behavioral patterns required to outpace decay on thousands of nodes. This is where the heartbeat correlation detection system becomes essential.

### 5.5 Heartbeat Correlation Detection: Why Scripted Behavior Cannot Hide

The heartbeat correlation system is designed to answer one question: is this node behaving like a human-operated machine, or like one instance of a scripted process running on shared infrastructure?

The answer is detectable because of fundamental physical and statistical properties that scripted behavior cannot convincingly fake at scale.

**The scripting problem.** If an attacker writes a script to maintain 1,000 nodes, all 1,000 instances of that script will exhibit correlated timing behavior. They will respond to network events in lockstep. They will send heartbeats at nearly identical intervals. They will receive broadcast messages within microseconds of each other if they share network infrastructure, because at the physical layer, a broadcast packet arriving at a shared network interface is delivered to all processes on that interface simultaneously, with no network path variation to introduce natural jitter. These patterns are statistically distinguishable from the behavior of 1,000 independent human operators running nodes on diverse hardware across diverse geographic locations.

**What the detector measures.** Three timing signals are analyzed:

*Broadcast arrival timing:* When a block is broadcast, all peers receive it at different times due to physical distance and network path differences. Legitimate validators show natural variance - tens to hundreds of milliseconds - in their arrival times relative to the broadcast event. Nodes on shared infrastructure show sub-millisecond variance because they share a network interface and receive the broadcast packet at the same physical moment.

*Heartbeat synchronization:* Legitimate validators running on diverse hardware with diverse system loads show naturally drifting clock behavior. Their periodic messages drift apart over time. Nodes controlled by a single script loop send heartbeats in near-perfect synchronization. Even with artificial random delays injected, the statistical distribution of inter-arrival times across thousands of events reveals non-random patterns.

*Reaction correlation:* When a network event occurs, legitimate validators react at different speeds based on their current CPU load and network conditions. Scripted nodes sharing a CPU react in lockstep. Cross-correlating reaction times across peer pairs across many independent events identifies this lockstep behavior with high confidence.

**Why this cannot be defeated at scale.** A sufficiently sophisticated attacker could inject artificial per-node timing variance to defeat each of these detectors individually. But doing so for 1,000 nodes, maintained continuously over 90 days, while also maintaining the behavioral merit requirements of each node, while also not letting the coordination between nodes become itself a detectable pattern - this is not a solvable problem with automation. The complexity compounds faster than any script can manage. The humans who would be required to oversee this at scale are themselves the bottleneck that the system is designed to create.

The heartbeat correlation system is implemented in `correlation.rs` and operates as a background analysis task during the full 90-day observer period. It produces confidence scores that feed into the NodeIntegrityScore described in Section 6.

---

## 6. Sybil Resistance Architecture

### 6.1 Overview

Sybil attacks are the fundamental adversarial problem for permissionless networks. A Sybil attack occurs when a single adversary creates multiple fake identities to gain disproportionate influence over a distributed system. The name comes from the 1973 book about a woman with dissociative identity disorder. The adversary presents many faces while being a single entity.

In blockchain networks, Sybil attacks take several forms. An attacker might spin up thousands of validator nodes to capture block production rewards. They might flood the peer network with fake nodes to disrupt gossip propagation. They might attempt to place their nodes in enough TPI slots to achieve 2-of-3 agreement on malicious blocks. Each of these attacks requires a different defense, and Valid Blockchain addresses all of them through a layered system.

### 6.2 Layer 1: Time and Merit Gating

The 90-day observation period with merit decay is the first and most fundamental layer. Any node can join the observer pool immediately. No node can produce blocks until it has been observed for 90 days and demonstrated sufficient behavioral merit.

Consider what this means for an attacker. They spin up 1,000 fake validator nodes on day 0. All 1,000 enter the observer pool. Now they must individually maintain each of those 1,000 nodes at a behavioral level sufficient to accumulate merit faster than decay. They must do this for 90 days, continuously, without the correlation detection system identifying the coordinated behavior of their nodes. At the end of 90 days, the nodes that passed all three requirements can promote to the validator pool.

The economics are stark. At USD 5 per month per VPS instance, 1,000 nodes costs USD 5,000 per month. At 90 days, that is USD 15,000 in VPS costs before a single node produces a block, and that is not even accounting for the time and cost of maintaining each of those nodes. The expected block reward capture from even a few hundred promoted nodes in a network with legitimate validators does not justify this expenditure, especially when the nodes are still competing with merit-accumulating legitimate validators for selection.

### 6.3 Layer 2: One IP per Identity

The network enforces one peer identity per public IP address. Two nodes sharing the same public IP hash to the same peer identity and are treated as a single peer.

**Example:** An attacker runs 10 validator processes on their home machine, all listening on different ports. From the network's perspective, all 10 nodes are the same peer, as they all share the same public IP. Only one of them can have a peer identity on the network. The other nine are effectively invisible.

This eliminates the cheapest class of Sybil attack entirely. A person cannot run 10 validators from one machine at home and expect 10 times the block reward. They get one validator's worth of influence.

To mount a Sybil attack, an adversary must use separate, unique public IP addresses for each fake node. This forces the attack into VPS infrastructure, converting it into a recurring financial cost as described in Section 6.2.

### 6.4 Layer 3: Heartbeat Correlation Detection

The correlation detection system is the most technically novel component of the Sybil resistance architecture. It is described in detail in Section 5.5. The following expands on the specific attack scenarios it addresses.

**Example - datacenter Sybil farm:** An attacker spins up 50 VPS instances on AWS us-east-1. All 50 instances are in the same availability zone, share the same network backbone, and receive broadcast packets within microseconds of each other. The broadcast arrival variance across these 50 nodes will be sub-millisecond. Legitimate validators distributed across the United States, Europe, and Asia will show arrival variance of 50-200 milliseconds for the same broadcast. The correlation detector identifies the 50 AWS nodes as a co-located cluster within the first few weeks of the observation period, long before they reach the 90-day promotion threshold.

**Example - distributed Sybil farm:** A more sophisticated attacker uses 50 VPS instances spread across multiple cloud providers and geographic regions. Broadcast arrival timing will now show realistic geographic variance. However, the heartbeat synchronization detector will still identify the scripted behavior. All 50 nodes send their periodic messages with timing controlled by the same script loop, producing a correlation signature across thousands of events over 90 days that is statistically distinguishable from 50 independent human operators.

**Example - identity rotation:** An attacker's nodes get flagged on day 15. They rotate all 50 node identities - generate new peer hashes and reconnect. The 90-day clock resets to zero. The correlation detector begins fresh analysis. Within days, the same timing fingerprint is detected from the same infrastructure. The new identities are flagged again. Rotation is not an escape. It is a way to extend the observation period indefinitely while paying VPS costs.

**Example - legitimate home validators:** Two friends on the same city block both run validator nodes on their home internet connections. Their broadcast arrival times will differ by 5-20 milliseconds due to different routers, different ISPs, and different physical distances to the same internet exchange point. Their heartbeat timing will differ due to different CPU loads, different background processes, different sleep schedules affecting when they interact with their nodes. The correlation detector does not flag them. The statistical separation between their timing profiles is well above the threshold for organic behavior.

#### 6.4.1 Broadcast Arrival Correlation

```
BroadcastAnchor {
    anchor_id:        String,
    slot:             u64,
    message_type:     String,
    emitted_at:       u64,
    block_hash:       Option<String>,
    broadcast_scope:  String,
    reference_points: Vec<String>,
}

PeerResponseSample {
    sample_id:        String,
    anchor_id:        String,
    peer_id:          String,
    observed_at:      u64,
    delta_ms:         u64,
    response_type:    String,
    rtt_ms:           Option<u64>,
    rtt_asymmetry_ms: Option<i64>,
}
```

The standard deviation of delta_ms across peer clusters over a rolling window is the primary correlation signal. Sub-5ms standard deviation across a cluster that shows 50-200ms individual deltas to other peers is a near-certain co-location indicator.

#### 6.4.2 Passive RTT Fingerprinting

Passive round-trip time measurements derived from normal protocol traffic create a latency fingerprint for each peer. Physics establishes hard floors that cannot be faked:

- Minimum transatlantic RTT (US to West Africa): approximately 120-150ms
- Minimum transcontinental RTT (US East to US West): approximately 60-80ms
- Minimum continental RTT within Europe: approximately 10-30ms

A node claiming to be in Lagos, Nigeria that shows 15ms RTT to a US-based bootstrap node is not in Lagos. The speed of light does not permit it. Such nodes are flagged as physics floor violations regardless of any other behavioral signals.

RTT asymmetry is an additional signal that distinguishes residential from datacenter infrastructure. Residential internet connections have asymmetric upload and download speeds, producing characteristic RTT asymmetry between outbound and inbound measurements. Datacenter connections are symmetric. This signal is collected passively from normal protocol traffic and contributes to the datacenter likelihood score in the RttLocationProfile.

```
RttLocationProfile {
    peer_id:                   String,
    reference_point_id:        String,
    rtt_samples_ms:            Vec<u64>,
    rtt_mean_ms:               f64,
    rtt_stddev_ms:             f64,
    rtt_jitter_score:          f64,
    rtt_asymmetry_ms:          f64,
    physics_floor_violation:   bool,
    location_confidence_score: f64,
    datacenter_likelihood:     f64,
    residential_likelihood:    f64,
}
```

Geographic diversity is an explicit design goal. Validators in underserved regions with demonstrably high RTT to major datacenters receive merit bonuses for contributing genuine geographic distribution to the network. A validator in rural Kenya whose RTT profile confirms their location is more valuable to network decentralization than ten validators in a Frankfurt datacenter, and the merit system reflects this.

### 6.5 Correlation Analysis Engine

```
CorrelationWindow {
    window_id:                  String,
    subject_id:                 String,
    window_start:               u64,
    window_end:                 u64,
    sample_count:               u64,
    arrival_stddev_ms:          f64,
    heartbeat_sync_score:       f64,
    reaction_correlation_score: f64,
    rtt_profile_score:          f64,
    location_confidence_score:  f64,
    cluster_id:                 Option<String>,
    cluster_size:               u64,
    confidence_trend:           String,
}
```

Minimum sample requirements before flagging:

- Hard minimum: 30-50 broadcast events per peer cluster
- Recommended: 100+ events for meaningful statistical confidence
- High confidence: 500+ events over the observation window

### 6.6 NodeIntegrityScore

```
NodeIntegrityScore {
    subject_id:                   String,
    current_confidence:           f64,
    network_timing_component:     f64,
    rtt_component:                f64,
    location_component:           f64,
    behavioral_component:         f64,
    wallet_component:             f64,
    cluster_component:            f64,
    promotion_blocking_threshold: f64,
    promotion_ready_threshold:    f64,
    recovery_eligibility:         bool,
    historical_flags:             Vec<FlagRecord>,
}
```

Current confidence is recoverable and an intended mechanic to promote sovereignty. "Kicking someone off the network" is not a sovereign response. Clean behavior over subsequent windows reduces the score. The historical flag record is permanent and append-only. A node with a clean current score but multiple historical flags is treated with more scrutiny than a node with no flag history.

### 6.7 Cluster Grouping and Group Penalties

When the correlation engine identifies a cluster of co-located nodes, all cluster members receive a simultaneous merit penalty large enough to require years of clean behavior to recover from. This is effectively a soft ban. Nodes remain on the network but are excluded from validator promotion for an extended duration.

The cluster grouping mechanic means that an attacker who successfully hides their identity rotation from the correlation detector still faces the group penalty on all nodes in the identified cluster. A single confirmed node drags the rest of the cluster down with it.

### 6.8 FlagPattern and Cross-Node Behavioral Matching

```
FlagPattern {
    subject_id:              String,
    total_flag_count:        u64,
    flag_timestamps:         Vec<u64>,
    flag_intervals_secs:     Vec<u64>,
    mean_interval_secs:      f64,
    interval_stddev_secs:    f64,
    cluster_ids_seen:        Vec<String>,
    repeated_cluster_count:  u64,
    peak_confidence_reached: f64,
    flags_post_recovery:     u64,
    pattern_class:           String,
    cross_node_matches:      Vec<String>,
    last_analyzed_at:        u64,
}
```

Attack campaigns have schedules. An attacker who maintains 1,000 nodes will typically run maintenance scripts, deploy updates, and restart nodes on a schedule. Two nodes with different network timing signatures that are flagged on the same days, at the same times, and in the same clusters across multiple observation windows are almost certainly operated by the same adversary. The FlagPattern system detects this cross-node correlation as a third independent Sybil resistance axis.

### 6.9 Attack Cost Analysis

| Attack Type | Infrastructure Cost | Time Cost | Outcome |
|-------------|--------------------|-----------|---------| 
| Same-IP home farm | $0 | Immediate | Blocked by 1-IP rule |
| Small VPS farm (10 nodes) | ~$50/month | 90 days minimum | Correlation flagged within weeks |
| Large VPS farm (1000 nodes) | ~$5,000/month | 90 days minimum | Operationally infeasible to maintain |
| Distributed farm (1000 nodes, diverse geo) | ~$8,000/month | 90 days minimum | Heartbeat sync detection, merit decay |
| 67% validator pool control (500 validators) | ~$1,000+/month | 90+ days | Requires 1,000+ maintained promoted nodes |

---

## 7. Zero Footprint Network Layer

### 7.1 Why Zero Footprint

Most network software collects more data than it needs and retains it longer than necessary. IP addresses are logged. Connection histories are stored. Peer behavior is tracked in persistent databases. This data accumulates and becomes a liability. A breach of the node's data store reveals information about every peer that ever connected.

Valid Blockchain takes the opposite approach. The Zero Footprint principle holds that the best privacy protection is the absence of data to protect. If a peer's IP address is never stored as an identity artifact, it cannot be leaked. If peer behavior is tracked only in memory and only for operational necessity, it cannot be recovered after the process exits.

This is not merely a privacy feature. It is a security feature with direct implications for the network's resilience. A validator node that stores nothing of value has nothing worth stealing. An adversary who compromises the node's storage finds no peer identity database, no connection history, and no behavioral logs. The Zero Footprint architecture limits the blast radius of any individual node compromise.

It also aligns with the lightweight design goal. Every byte not stored is a byte that does not consume RAM, disk, or backup capacity. A node that holds its state entirely in memory with no persistence requirements beyond the archive segments is fundamentally simpler to operate than one managing multiple databases and log files.

### 7.2 Peer Identity Model

Raw IP addresses are never stored as peer identity. A peer's identity is derived through a deterministic, epoch-salted hash:

```
epoch_number = (current_time - genesis_timestamp) / EPOCH_DURATION_SECONDS
epoch_salt   = SHA256(genesis_hash || epoch_number_bytes)
peer_hash    = SHA256(epoch_salt || canonicalized_peer_address_bytes)
```

The epoch salt rotates every 24 hours. Peer hashes change daily, preventing long-term correlation of peer behavior to specific addresses.

The identity and transport layers are cleanly separated:

```rust
peers:        HashMap<String, PeerInfo>  // keyed by peer_hash
dial_targets: HashMap<String, String>   // peer_hash -> raw transport address
```

Gossip distributes raw transport addresses. Logs record hashes only. No component persists a raw IP address as an identity artifact.

### 7.3 Address Canonicalization

All inbound peer addresses are canonicalized before hashing:

```rust
pub fn canonicalize_peer_addr(addr: &str) -> Option<String> {
    // 0.0.0.0:port -> actual transport IP:port
    // 127.0.0.1 -> normalized localhost
    // [::1] -> normalized IPv6 localhost
    // Hostnames -> lowercased
    // []:port -> rejected (empty bracketed host)
    // Returns None if address is malformed
}
```

Malformed addresses that fail canonicalization are dropped before hashing. A non-address string never becomes identity material.

### 7.4 TLS 1.3 Transport

All peer-to-peer connections are encrypted with TLS 1.3. Certificates are ephemeral, and generated in memory at startup and discarded on shutdown. They are never persisted to disk.

```rust
pub fn generate_tls_config() -> Arc<ServerConfig> {
    let cert = rcgen::generate_simple_self_signed(vec!["valid-node".to_string()])
        .expect("cert generation failed");
    // Certificate exists only in memory.
    // Never written to disk. Never persisted across restarts.
    Arc::new(build_server_config(cert))
}
```

Certificate fingerprints are logged for observability. The `trusted_peer_fingerprints` configuration field allows operators to specify a SHA-256 allowlist. All outbound connections and broadcasts enforce this allowlist when configured.

### 7.5 Network Abuse Hardening

Inbound connection rate limiting at 5 attempts per 60-second window per source IP prevents flooding before TLS handshake overhead is incurred:

```rust
fn check_connection_rate(ip: &str, state: &ConnectionRateState) -> bool {
    let window = state.get_window(ip);
    window.count < MAX_CONNECTIONS_PER_WINDOW
}
```

Per-peer message rate limiting at 100 messages per 10 seconds disconnects flooding peers immediately. The rate check runs before peer liveness is updated, ensuring abusive messages do not mutate peer state.

---

## 8. Archive, Persistence, and Defeating Chain State Bloat

### 8.1 The Chain State Bloat Problem

Chain state bloat is one of the most consequential unsolved problems in blockchain design. Bitcoin's UTXO set and full blockchain history now require hundreds of gigabytes of storage. Ethereum's state has grown so large that running a full archival node requires terabytes. Both networks have responded with various workarounds - pruned nodes, light clients, stateless clients, but these all involve tradeoffs that compromise either decentralization or security.

The root cause is straightforward: every transaction ever executed leaves a permanent mark on the chain state, and that state grows monotonically with usage. More users means more state. More transactions means more state. A popular blockchain is a bloating blockchain, and the bloat is directly proportional to its success.

Valid Blockchain solves this through a combination of in-memory state management and the 6-hour archive segment model. The solution is architecturally elegant and does not require any of the tradeoffs that have plagued other approaches.

### 8.2 In-Memory State

The complete chain state of Valid Blockchain lives in memory during operation:

```rust
pub struct ChainState {
    pub blocks:       HashMap<u64, Block>,     // slot -> block
    pub balances:     HashMap<String, u64>,    // address -> nanoVLid
    pub nonces:       HashMap<String, u64>,    // address -> current nonce
    pub total_supply: u64,
    pub latest_slot:  u64,
    pub genesis_hash: String,
}
```

No external database. No disk writes during block production. The entire chain state is the process memory. This has several important consequences:

- A fresh node can reconstruct the current state from the archive segments and peer sync without any local persistent storage
- There is no database process to maintain, back up, or secure
- State reads and writes are pure memory operations with no I/O overhead
- The node's storage footprint is bounded by the archive segment retention policy, not by the total transaction history

### 8.3 6-Hour Archive Segments: How Bloat Is Defeated

Every 2,160 blocks (approximately 6 hours at 10-second block times), the retiring block range is written as a durable archive segment and pruned from memory.

```
ArchiveSegment {
    metadata: ArchiveMetadata {
        genesis_hash:         String,
        segment_start_slot:   u64,
        segment_end_slot:     u64,
        block_count:          u64,
        payload_checksum:     String,
        archive_version:      u32,
    },
    blocks: Vec<Block>,
}
```

After the archive segment is written and verified, the blocks it contains are pruned from the in-memory HashMap. The process then has only the most recent blocks in memory, not the entire history.

This is how chain state bloat is defeated. A node that has been running for a year does not hold a year of blocks in memory. It holds the last six hours of blocks in memory, plus a set of archive segments on disk covering the preceding history. The in-memory footprint is permanently bounded. Adding more validators, processing more transactions, and running the network for more years does not increase the RAM requirement for a validator node. The hardware requirements do not grow with the network.

The archive segments are the chain's long-term memory. They are not restore checkpoints in the traditional sense. They are a permanent historical record from which any period of chain history can be reconstructed. But a validator does not need the full historical record to participate in the current consensus. Peer-based live sync provides catch-up from the current chain head without requiring local access to historical archive segments.

```rust
// Archive generation runs without holding the chain state lock
tokio::task::spawn_blocking(move || {
    let segment = build_archive_segment(&block_range);
    write_archive_segment(&segment, &path)?;
    load_verified_archive_segment(&path)?;
    Ok(())
});
// Chain lock is only briefly re-acquired for the prune step
```

Archive generation, writing, and verification all run without holding the chain state lock. The chain continues producing blocks while the archive is being written. File I/O is isolated via spawn_blocking so it does not block the async runtime. Only the brief prune step requires the write lock.

### 8.4 Arweave Publication

After each verified local archive segment, a publication manifest is queued for permanent off-chain storage on Arweave:

```
BroadcastAnchor tags:
App-Name:          accessible-tpi-chain
App-Version:       v0.8.0
Content-Type:      application/json
Archive-Type:      block-segment
Chain-Genesis:     <genesis_hash>
Segment-Start:     <start_slot>
Segment-End:       <end_slot>
Segment-Checksum:  <checksum>
Archive-Version:   <version>
```

The Arweave upload pipeline implements the full format 2 transaction specification with correct deep hash computation, RSA-PSS signing, and data_root Merkle tree construction. Transaction correctness was validated live on the Arweave mainnet with transaction ID `71o-aNdFvGGPEcIvK6b4MCFKQjS-FJD_-KAIKDeiKCA`.

Prune correctness never depends on upload success. Local durability always gates prune. **The Arweave backend is designed to be replaceable by VIPFS when that layer reaches production,** but will still be used for disaster recovery scenarios.

### 8.5 Peer-Based Live Sync

On startup, a node queries configured peers for the current chain head and fetches missing blocks sequentially via the `/head` and `/block/:slot` RPC endpoints. Production begins only after successful catch-up. A node that fails to sync exits cleanly.

---

## 9. Token Economics

### 9.1 VLid Supply Model

VLid (pronounced "valid") is the native token of Valid Blockchain. The supply model is designed around one principle: tokens mint only when validated work is proven.

- **Total cap:** 33 million VLid
- **Decimal places:** 9 (1 VLid = 1,000,000,000 nanoVLid)
- **Emission timeline:** 21 years across 3 epochs of 7 years each
- **Minting trigger:** Block production only

There is no pre-mine, VC allocations, foundation treasury, and no team reserve. That would put the blockchain's **users** in debt before the blockchain even launches. The genesis bootstrap allocation exists solely to seed the initial validator set and is the **only** non-block-production source of initial supply.

### 9.2 Emission Schedule

| Epoch | Years | Fraction of Total Supply |
|-------|-------|--------------------------|
| 0     | 0-7   | 60%                      |
| 1     | 7-14  | 30%                      |
| 2     | 14-21 | 10%                      |

Within each epoch, block rewards are fixed and emitted per block produced. The Epoch 0 block reward is 0.0808 VLid per block. Rewards decay at epoch boundaries, not continuously, providing predictable emission curves.

### 9.3 Block Rewards and Fees

Block rewards are paid to the validator that produces the block. Fees collected from transactions in a block are also paid to the block producer. The fee distribution model is temporary and will be properly sorted in v0.9.x, before mainnet launch.

The reward minting function enforces the supply cap strictly:

```rust
fn mint_block_reward(state: &mut ChainState, producer: &str, reward: u64) {
    let available = TOTAL_SUPPLY_CAP - state.total_supply;
    let actual_reward = reward.min(available);
    *state.balances.entry(producer.to_string()).or_insert(0) += actual_reward;
    state.total_supply += actual_reward;
}
```

When the cap is reached, validator compensation transitions entirely to transaction fees.

### 9.4 Genesis and Validator Reward Allocations

The genesis allocation and specific validator reward parameters are subject to adjustment in v0.9.x as the validator set size and network characteristics become known through testnet operation. The distribution principles are as follows: merit-based, no pre-mine, no team reserve, and are fixed. The specific numerical parameters will be finalized before mainnet launch based on testnet data.

### 9.5 Governance

Governance of Valid Blockchain is merit-based. Voting weight is determined by validator merit score and wallet age, not token balance. This prevents token-weighted governance from concentrating decision-making power among large holders.

Ecosystem grants are being proposed as a formal governance mechanism. Grants would be denominated in VLid and allocated from transaction and on-chain fees, funding development work that strengthens the Valid ecosystem. Approved grants would be distributed on a milestone basis, with payment contingent on verified delivery. This model avoids the speculation and misalignment of traditional VC funding while providing sustainable support for ecosystem development.

---

## 10. Testnet

### 10.1 Purpose

The testnet serves two distinct purposes that are both essential to the mainnet launch.

The first is technical: finding and fixing bugs under real network conditions with real participants, real hardware diversity, and real adversarial pressure. Code that works in a local test environment often behaves differently on a network with dozens of nodes across diverse geographic locations, internet service providers, and hardware configurations. Phase 4 of the testnet involves self-coordinated attacks where participants actively attempt to break the network. Every bug found during testnet will be fixed before mainnet.

The second is merit bootstrapping. As described in Section 4.1, mainnet cannot launch without an established validator set. The testnet period is the mechanism by which the initial validator set earns and demonstrates the merit that will carry forward to mainnet. A validator who participates consistently and contributes meaningfully to testnet development arrives at mainnet launch with a verified 6-month behavioral record. This is not incidental, but rather the intended design.

### 10.2 Testnet Phases

**Phase 1 - Network and Connectivity**

Establishing that the network runs without bootstrap reliance. Bootstrap nodes are brought up at distinct physical locations. Community validators connect. The critical test is whether the network continues producing blocks and propagating gossip after the bootstrap nodes are taken offline. A network that depends on its bootstrap nodes for liveness is not ready for mainnet.

- Bootstrap node independence verification
- Identity protection and connection security hardening under live peer conditions
- Connection stability across varied hardware and network environments
- Rate limiting and abuse protection under live traffic

**Phase 2 - CLI Wallet and Transaction Health**

Establishing that the full transaction lifecycle works correctly under live usage patterns.

- Wallet connectivity and transaction signing
- Transaction submission, mempool behavior, and propagation
- Block production and transaction inclusion verification
- Nonce handling, replay protection, fee priority, and mempool limit behavior
- RPC endpoint stability and error handling
- Block production reward minting and edge cases

**Phase 3 - Merit Hardening**

Implementing and testing the merit system under real network conditions with real behavioral data.

- Validator block production merit accumulation and decay
- Behavioral merit implementation and integration with real network data
- Quarantine mechanic verification under simulated misbehavior
- Merit scoring edge cases identified and resolved

**Phase 4 - Stress Testing**

Deliberately attempting to break the network.

- Sustained high-volume transaction load
- Self-coordinated attacks and adversarial edge case pushing
- Peer churn, mempool saturation, validator dropout, and network recovery
- Archive segment generation under continuous block production

**Phase 5 - Final Staged Features**

Reserved for features and fixes surfaced during earlier phases, community feedback, proper fee distribution, and pre-mainnet hardening.

### 10.3 Duration and Timeline

The testnet is expected to run for approximately six months. Six months provides sufficient time to:

- Run meaningful stress tests across all five phases
- Accumulate enough behavioral data to calibrate the correlation detection thresholds before mainnet
- Allow validators sufficient time to establish genuine merit standing
- Surface and fix the class of bugs that only appear under sustained real-world operation

### 10.4 Testnet Rewards

Testnet participation is rewarded through two mechanisms.

**Genesis allocation distribution.** The genesis bootstrap allocation of VLid will be distributed to testnet participants based on their leaderboard performance at the conclusion of the testnet period. Leaderboard standing is tracked by the Observer Bot in the Discord server, which records attendance, participation, bug finds, bug fixes, notable contributions, social media posts, articles, videos, and test completions. Points are awarded manually by myself, Rook, on verified activity. The Observer Bot is testnet scaffolding. It will be retired after mainnet launch.

**Bootstrap merit.** The behavioral merit accumulated during testnet carries forward to mainnet as each validator's starting standing. This is not a small reward. A validator who has participated consistently for six months arrives at mainnet with a verified behavioral history that gives them genuine selection priority over any fresh node that connects at mainnet launch. This bootstrap merit is why testnet participation is valuable beyond any token reward. It is the foundation of each validator's mainnet standing.

The combination of these two rewards creates a strong alignment between testnet participation quality and mainnet success. The validators who contribute most to making testnet succeed are the ones who arrive at mainnet in the strongest position, ready to produce.

---

## 11. Scalability: Why Valid Blockchain Scales Without Slowing Down

### 11.1 The Scalability Trilemma and How TPI Sidesteps It

Blockchain systems are commonly said to face a trilemma between decentralization, security, and scalability, the claim being that improving any two requires sacrificing the third. Valid Blockchain's architecture challenges this framing directly, particularly regarding the relationship between validator count and performance.

In committee-based consensus systems like PBFT and its derivatives, adding validators increases the number of messages that must be exchanged to reach consensus. A 100-validator committee requires an order of magnitude more communication than a 10-validator committee. Performance degrades as decentralization increases.

TPI does not have this property. The validator selection step is a local computation:

```rust
let selected = select_tpi_validators(slot, &validator_pool);
```

This computation is performed independently by each node. It requires no network communication. Adding 1,000 validators to the pool does not add any communication overhead to this step. The three selected validators then exchange hashes, exactly three messages regardless of pool size. The consensus round is always three-party, always the same communication overhead, regardless of whether the pool contains 10 validators or 10,000.

### 11.2 Why More Validators Means More Security, Not Slower Consensus

As established in Section 4.3, adding validators to the pool increases the unpredictability of selection and increases the cost of a Sybil attack superlinearly. A network with 1,000 validators requires an attacker to maintain approximately 2,000 promoted Sybil nodes to achieve 67% pool control. A network with 100 validators requires only 200.

At the same time, the consensus round remains exactly three-party. Block production time does not change. Finality time does not change. The TPI window, buffer window, and racer window are fixed at startup and do not scale with validator count.

The result is that Valid Blockchain becomes more secure as it grows, without becoming slower. This is the opposite of the common pattern in blockchain systems where growth leads to either centralization pressure or performance degradation.

### 11.3 Horizontal Scalability Through Layer 2

The L1 is deliberately designed to be stable and conservative. High-throughput transaction processing, complex smart contract execution, and experimental consensus variants all belong at Layer 2. The L1 provides a secure, reliable settlement layer. Layer 2 networks built on Valid provide the scalability surface.

This model mirrors successful patterns in other distributed systems. The L1 is the invariant foundation. Layer 2 handles the variable workload. Neither layer needs to solve problems that belong to the other.

### 11.4 Chain State Scalability

As described in Section 8, the archive segment model ensures that the in-memory footprint of a validator node does not grow with network usage. A node running for ten years holds approximately the same amount of data in memory as a node running for one day - only the most recent six hours of blocks. Historical data lives in archive segments and on Arweave, accessible but not consuming validator RAM.

This means the hardware requirements for running a Valid Blockchain validator node do not increase with network adoption. A 4GB Raspberry Pi that can run a node today can run a node when the network has a million transactions per day. The 2GB RAM minimum is not a temporary concession to current usage. It is a permanent architectural guarantee.

---

## 12. Governance

### 12.1 Merit-Based Voting

Governance of Valid Blockchain is merit-based. Voting weight is determined by a combination of validator merit score and wallet age, not token balance. This design prevents the pattern seen in token-weighted governance systems where large holders accumulate disproportionate influence, effectively recreating the plutocratic structures that blockchain technology was supposed to replace.

A validator who has participated consistently for two years and contributed meaningfully to the network has more governance weight than a wallet that recently acquired a large token balance without any participation history. This aligns governance influence with demonstrated commitment to the network rather than financial resources.

### 12.2 Ecosystem Grants

Ecosystem grants are being proposed as a formal governance mechanism within the Valid ecosystem. The grant model works as follows:

Developers or teams propose a project that would benefit the Valid ecosystem. The proposal includes a technical specification, a milestone plan, and a requested grant amount denominated in VLid. The validator community votes on the proposal using merit-weighted voting. Approved proposals receive milestone-based disbursements. Payment is contingent on verified delivery of each milestone, not on submission of the proposal.

This model is designed to fund the kind of work that makes the ecosystem stronger without creating the misalignment of traditional VC funding. A VC-funded project has obligations to its investors. A grant-funded project has obligations to the community that funded it. The incentives are aligned differently, and the resulting software is typically more aligned with community needs.

### 12.3 Protocol Parameter Governance

Protocol parameters including merit decay rates, promotion thresholds, correlation detection sensitivity, and fee distribution will be subject to community governance vote before and after mainnet launch. Parameters will be adjusted based on observed network behavior during testnet, with the final values confirmed by validator vote before the mainnet genesis.

---

## 13. The Fork Philosophy: Valid as an Ecosystem

### 13.1 Built to Be Forked

Valid Blockchain is explicitly designed to be forked. The `protocol` branch of the accessible-tpi-chain repository is a clean, documented, dependency-vendored starting point for anyone who wants to build a blockchain on the TPI consensus foundation without inheriting Valid Blockchain-specific implementation decisions.

This is not a concession or an afterthought. It is a deliberate design goal that shapes every architectural decision. The codebase is compact by design. The consensus, networking, and state management components are cleanly separated. The configuration system is flat and documented. A developer with a solid Rust background should be able to read the codebase, understand it, and begin adapting it within days.

### 13.2 The Linux Model

The relationship between the Valid protocol and its forks is intended to mirror the relationship between the Linux kernel and Linux distributions. The kernel is maintained by a core team and provides a stable, well-tested foundation. Distributions make different choices about what to include, how to configure the system, and who their audience is. Ubuntu makes different choices than Arch Linux. Red Hat makes different choices than Debian. All of them benefit when the kernel improves. All of them contribute back improvements that benefit the whole ecosystem.

Valid Blockchain is the kernel. The `protocol` branch is the source from which forks are made. The `valid-blockchain` branch is one distribution - the public blockchain with merit-based Sybil resistance and open validator participation. The `private-network` branch is another distribution - permissioned, fixed-validator, stripped of public-chain assumptions. Future forks by other teams will be additional distributions.

Each distribution strengthens the ecosystem. A hospital that forks Valid for medical record coordination contributes to the real-world validation of the TPI consensus mechanism. A gaming company that forks Valid for in-game economy settlement contributes to understanding the performance characteristics of the system under high transaction volume. A community in a developing country that forks Valid for local financial infrastructure contributes to the geographic diversity of the ecosystem.

### 13.3 Ecosystem Grants and Fork Incentives

Ecosystem grants serve a dual purpose in this model. They fund development work that benefits the Valid Blockchain directly. They also fund development work that benefits the broader ecosystem of Valid-derived chains. A tool that makes it easier to fork and configure a Valid-based chain benefits every chain in the ecosystem. A library that improves the TPI consensus implementation benefits every fork that has not diverged too far from the upstream.

This creates a virtuous cycle. Successful forks validate the architecture. Grant-funded improvements to the architecture benefit all forks. The ecosystem grows in ways that individual forks would not fund independently. This is precisely the dynamic that has made Linux successful across an extraordinarily diverse range of applications.

### 13.4 Private Network Branch

The `private-network` branch is a production-ready deployment product for closed and permissioned environments. It strips the public-chain security assumptions - merit hardening, observer/validator tier separation, Arweave publication - that are unnecessary when the validator set is known and trusted. It retains the TPI consensus, Ed25519 signatures, TLS transport, mempool, RPC, and in-memory state model.

The `private-network` branch is documented for operators, administrators, and backend developers rather than blockchain community members. It supports fixed-validator topologies, configurable persistence modes, and stable deployment assumptions rather than the dynamic, adversarial assumptions of the public chain.

### 13.5 Fork Use Cases

The following describes concrete deployment scenarios for Valid-derived chains. Each represents a real organizational need that the TPI consensus architecture is well-suited to address.

**Hospital and Healthcare Networks**

Patient record coordination across a hospital system requires a distributed ledger that can be audited, does not depend on any single party's infrastructure, and does not expose patient data to external networks. A Valid fork configured as a private network with three or more hospital-operated validator nodes provides this. The blockchain records access events, prescription records, and test results with immutable timestamps. No public chain is involved. No tokens are required. The TPI consensus provides Byzantine fault tolerance within the trusted hospital network.

**Warehouse and Supply Chain Management**

Inventory tracking across a large distribution network involves multiple parties - suppliers, warehouses, logistics companies, retailers - who need a shared ledger they can all trust. A Valid fork with permissioned validator access for each party provides a neutral coordination layer. Inventory movements are recorded as transactions. Disputes are resolved by querying the immutable chain record. The in-memory architecture handles the high transaction volume of a busy warehouse without requiring database infrastructure at each location.

**Corporate and Enterprise Backends**

Internal financial reconciliation, audit trails, and multi-department approval workflows can all be implemented on a Valid-derived private chain. The TPI consensus provides multi-party agreement without requiring any single department to be the trusted authority. A finance department, legal department, and operations department each running a validator node cannot individually falsify records. All three must agree.

**Gaming Infrastructure and Economy**

In-game economies require a ledger that is fast, cheap to operate, and resistant to manipulation by either the game operator or players. A Valid fork configured for high-throughput private operation can handle in-game asset transfers, marketplace transactions, and achievement records with sub-second finality. The archive model keeps the chain state lightweight regardless of transaction volume. The game operator controls the validator set, maintaining operational control while providing players with an auditable, tamper-evident record.

**Custom Public Blockchains**

Teams that want to launch a public blockchain with TPI consensus but their own token economics, governance model, or application layer can fork from the `protocol` branch. They inherit a working, tested consensus mechanism and network layer and build their application-specific logic on top. The ecosystem grant model could fund development of tooling that makes this fork-and-customize workflow easier.

**Niche and Community Blockchains**

A neighborhood cooperative, a community land trust, a local credit union, an automated mining facility located in harsh conditions, or a small-scale cooperative economy could run a Valid-derived chain for their specific community. The 2GB RAM minimum makes this accessible to communities without significant technical resources. The open-source license means there is no software cost. The TPI consensus means no single community member has unilateral control over the ledger.

---

## 14. Future Work

### 14.1 Correlation and Integrity Implementation

The `correlation.rs`, `behavioral_merit.rs`, and `integrity.rs` modules are currently scaffolded with complete data structures and function signatures. Full implementation begins after live testnet deployment is stable and real-world behavioral data can be used to calibrate detection thresholds. The specification is documented in `docs/v0.8-correlation-spec.md`.

### 14.2 VNS (Valid Name Service)

VNS is a domain registry Layer 2 built on the same minimal architectural principles as the L1. It is in active development and will be released post-v1.0.

VNS witnesses L1 payments through a light-client style verification mechanism rather than a bridge. There is no cross-chain bridge, no wrapped token, and no external oracle. The naming model uses own-forever registration rather than recurring renewal, eliminating the DNS renewal failure mode where domains are lost due to administrative oversight.

Premium name policy and anti-spam economics will be defined during implementation. Any governance or blacklist enforcement will be narrowly scoped and explicitly documented. VLid is the native settlement token. **No separate VNS token is planned.**

VNS creates a human-readable namespace for Valid Blockchain addresses, smart contracts, and services. A hospital running a Valid private chain can register `hospital.vns` and expose their RPC endpoint under that name. A developer deploying a contract can register `myapp.vns`. The naming layer makes the ecosystem significantly more accessible without any tradeoff in decentralization.

### 14.3 VIPFS (Valid IPFS)

VIPFS is a distributed content seeding and retrieval layer that will be released post-v1.0. It is in active design and early development.

VIPFS serves two purposes. As a standalone layer, it provides distributed content storage and retrieval for the Valid ecosystem - applications, websites, media, and arbitrary data can be hosted on VIPFS with cryptographic content addressing and distributed availability guarantees. As a backend for the archive publication sidecar, it will eventually replace Arweave as the permanent storage layer for Valid Blockchain archive segments.

The publication sidecar is already designed with a replaceable backend interface. When VIPFS reaches production readiness, switching the archive publication backend from Arweave to VIPFS requires no changes to the archive or chain state logic. The transition will be seamless for validators.

VIPFS validators and users contribute storage and bandwidth based on what they store and access, creating a natural economic alignment between content demand and supply. Content moderation and illegal-content handling will be explicitly defined before implementation. This is not deferred.

### 14.4 KEVIN (Distributed AI Inference)

KEVIN is a distributed inference marketplace aligned with the broader Valid stack. It will be released after VNS and VIPFS have established the naming and storage layers.

KEVIN enables validators and users to contribute compute capacity for AI inference tasks and receive VLid compensation. The execution model spans local inference, peer-provided compute, and specialized validator hardware, matching inference requests to available capacity. Hardware requirements, pricing, and validator reward models will be defined when the architecture is formalized.

KEVIN is not a separate token. Again, settlement uses VLid as the native token. This avoids the proliferation of ecosystem-specific tokens that fragment liquidity and complicate the economic model.

K.E.V.I.N. currently exists as a Discord bot with Ollama integration and Valid Blockchain awareness. The production KEVIN network is the distributed evolution of that concept. Community members can interact with K.E.V.I.N. in the Discord server today and contribute to its development.

### 14.5 Valid Browser

Valid Browser is a fork of Brave with Valid-native integrations built in at the browser level. It is in active development.

The browser provides built-in L1 wallet support as a first-class feature rather than an extension. It natively resolves Valid Name Service addresses, allowing users to navigate to `app.vns` in the URL bar. It provides native access to VIPFS content without requiring any configuration. It supports direct L1 payment signing and application-layer interactions without exposing private keys to web content.

The initial scope is constrained to Valid network integrations. The browser will not attempt to replicate the full Brave feature set immediately. It will add Valid-specific capabilities on top of the Brave base and expand from there.

### 14.6 Valid Terminal

Valid Terminal is a security-hardened terminal emulator forked from Alacritty, in active development. It includes homoglyph attack filtering in both keyboard input and paste paths, preventing a class of attack where visually similar Unicode characters are used to disguise malicious commands.

### 14.7 Valid Vault

Valid Vault is a **local** password manager with WebAuthn fingerprint authentication and encrypted sync, designed for the Valid ecosystem.

### 14.8 Private Network Branch Development

The `private-network` branch will continue to be developed as a production-grade deployment product. Planned additions include configurable persistence modes (memory, local archive, full), fixed-validator topology configuration, simplified RPC endpoints for non-blockchain developers, and comprehensive operator documentation covering deployment, maintenance, and security assumptions.

### 14.9 Anti-Frontrunning Formalization

The parent block hash seeding for transaction ordering is implemented but not yet formally specified. A formal specification of the anti-frontrunning properties, including a proof that the ordering cannot be predicted by the block producer before the parent block is finalized, will be produced before mainnet launch.

---

## 15. Security Analysis

### 15.1 Threat Model

Valid Blockchain's threat model targets the following adversary classes:

**Casual adversary:** Limited technical resources, no sustained operational capacity. Defeated by one-IP-per-identity rule and the observation period requirement.

**Moderately resourced adversary:** Access to VPS infrastructure, some technical sophistication. Defeated by correlation detection, merit decay, and the operational impossibility of maintaining hundreds of nodes above the merit threshold simultaneously.

**Well-funded adversary:** Significant capital, technical team, sustained operational capacity. Substantially impeded by the time requirement, behavioral merit requirements, correlation detection, and the superlinear cost of achieving validator pool control as the honest pool grows.

**Nation-state adversary:** Unlimited resources and patience. No system defeats this class of adversary unconditionally. The Valid Blockchain threat model makes no claims of resistance against unlimited-resource adversaries and explicitly does not target this class.

### 15.2 TPI Collusion Attack

The most serious attack is a TPI collusion attack. A Sybil farm achieves sufficient validator pool penetration to consistently place two of its nodes in the same TPI slot, allowing them to agree on a malicious block.

To guarantee consistent 2-of-3 selection at pool size N, an attacker needs approximately 2N promoted Sybil validators. As N grows, the attack cost grows with it. A network with 500 legitimate validators requires the attacker to maintain approximately 1,000 promoted Sybil nodes simultaneously, each passing correlation detection and maintaining behavioral merit above the promotion threshold. This is operationally infeasible under the merit decay and heartbeat correlation system.

### 15.3 Comparison with PoW and PoS

| Property | PoW | PoS | Valid (TPI + Merit) |
|----------|-----|-----|---------------------|
| Sybil resistance mechanism | Compute cost | Capital cost | Time + behavior |
| Minimum participation barrier | High hardware cost | High capital cost | 2GB RAM |
| Bypassable with money | Yes | Yes | No - time cannot be purchased |
| Geographic concentration | Yes | Yes | Actively incentivized against |
| Attack cost scales with network size | Weakly | Weakly | Superlinearly |
| Excludes legitimate participants | Yes | Yes | No |
| Security grows with validator count | No | No | Yes |

---

## 16. Conclusion

Valid Blockchain demonstrates that a permissionless, accessible, and genuinely decentralized blockchain is achievable without capital requirements, compute requirements, or growing hardware demands. TPI consensus provides sub-second finality on 10-second blocks without committee overhead. The merit system provides Sybil resistance that scales with validator pool size and is denominated in time and behavior rather than money. The archive model permanently defeats chain state bloat. The Zero Footprint network layer protects validator privacy by architecture rather than policy. The 2GB RAM minimum is not a limitation. It is the point.

The system is pre-mainnet and in active testnet development. The correlation and integrity systems are scaffolded and will be implemented and calibrated during the testnet period using real behavioral data. Every component described in this paper is implemented and testable in the accessible-tpi-chain repository.

The fork philosophy, the Linux-model ecosystem, and the ecosystem grant program are designed to ensure that Valid Blockchain's value extends beyond any single deployment. The protocol is a foundation. What gets built on it is limited only by what communities need and developers imagine.

Through the grace of God, this is my best effort at a distributed system that does not destroy the environment, does not cater to corporations and/or the rich, and does not exclude the people it was built for.

---

## References

- Nakamoto, S. (2008). Bitcoin: A Peer-to-Peer Electronic Cash System.
- Buterin, V. et al. (2022). Ethereum Proof of Stake specification.
- Castro, M. and Liskov, B. (1999). Practical Byzantine Fault Tolerance. OSDI.
- Douceur, J. (2002). The Sybil Attack. IPTPS.
- Torvalds, L. et al. Linux kernel source. https://kernel.org
- Arweave format 2 transaction specification. https://docs.arweave.org
- RustSec Advisory Database. https://rustsec.org
- Accessible TPI Chain repository. https://github.com/HiImRook/accessible-tpi-chain

---

*Copyright (c) 2024-2026 by Rook. MIT License.*
