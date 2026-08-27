# Valid Blockchain: A Plain Language Guide

**Author:** Rook  
**Version:** 1.0  
**Date:** August 27, 2026  
**Website:** https://hiimrook.github.io/accessible-tpi-chain/  
**Repository:** https://github.com/HiImRook/accessible-tpi-chain  
**Discord:** https://discord.gg/2SP383cJs9

---

## What Is Valid Blockchain?

Valid Blockchain is a new type of blockchain built from scratch. It does not fork from Ethereum, Bitcoin, or anything else. Every piece of it was written by hand in Rust, with one goal above all others: make it possible for anyone in the world to participate as a validator, regardless of how much money they have or what hardware they are running.

The two ideas at the heart of it are worth understanding before anything else.

**Three-Party Integrity (TPI)** is the consensus mechanism that powers Valid Blockchain. Instead of requiring thousands of validators to communicate and vote, TPI selects exactly three validators per block. Each of them independently computes the same block and compares results. If they agree, the block is finalized in under a second. If one of them disagrees, that validator gets penalized. It is fast, it is efficient, and it does not slow down as the network grows.

**Zero Footprint** is the philosophy behind how the network handles identity and data. A validator's IP address is never stored. Peer identity is derived from a hash that rotates daily. Certificates are generated in memory and thrown away when the node shuts down. The network is designed so that there is nothing sensitive to steal, because nothing sensitive is ever held.

---

## The Problem Valid Blockchain Is Solving

Bitcoin mining is dominated by a small number of industrial mining operations. Ethereum requires 32 ETH to run a validator, which is a significant financial barrier for most people on Earth. Both of these networks were built with decentralization as a stated goal, but the economic reality is that participation is concentrated in wealthy countries and among people who can afford the entry cost.

Valid Blockchain rejects that model entirely. There is no stake required. There is no specialized hardware required. A Raspberry Pi with 4GB of RAM can run a full validator node. A laptop from the last decade can run a full validator node. Someone in a rural area with a modest internet connection is a first-class participant here, not an afterthought.

The other problem both major networks face is that they get heavier over time. Running a full Bitcoin node requires hundreds of gigabytes of storage. Running a full Ethereum node requires terabytes. The more popular these networks get, the harder it becomes to participate in them. Valid Blockchain solves this by design, not as a future plan. The architecture keeps memory requirements permanently bounded no matter how long the network runs or how many transactions it processes.

---

## How TPI Consensus Works

Every 10 seconds, a new block slot begins. Three validators are selected from the active pool using a seeded hash of the slot number. This selection is random but deterministic, meaning every validator can independently verify who was selected without any communication happening first.

Each of the three selected validators independently builds the next block from the current transaction pool and computes a hash of it. They share those hashes with each other. If all three match, the validator with the highest merit score produces and broadcasts the block. If two of three match, the same thing happens, but the one who submitted a different hash receives a penalty. If none of them agree, a backup validator called the racer steps in and produces the block. If the racer also fails, an empty block is produced on schedule so the chain never stalls.

The reason this works without slowing down as the network grows is that the consensus round is always between exactly three validators. Selecting who those three are is a local computation that each node runs independently. Adding a thousand more validators to the pool does not add any communication overhead to the process. It just makes the selection harder to predict and harder to manipulate, which makes the network more secure.

---

## What Merit Is and Why It Matters

Merit is how Valid Blockchain determines who gets to produce blocks and who gets to participate as a validator. It replaces the role that money plays in Proof of Stake systems and that hardware plays in Proof of Work systems. A validator earns merit by showing up consistently and behaving honestly. A validator loses merit by misbehaving or going inactive for extended periods.

There are two kinds of merit that work together.

**Validator merit** is earned by participating correctly in the TPI consensus process. Producing correct blocks, agreeing with the consensus outcome, and serving as a reliable backup when called upon all build validator merit. Submitting a wrong hash or failing to respond when selected reduces it.

**Behavioral merit** is earned by being a genuine, active participant over time. This includes keeping your node online consistently, having a wallet with real transaction history, and contributing to the network in ways that automated scripts cannot convincingly fake over a long period.

The reason merit decay exists is important to understand. Without decay, someone could run a node aggressively for a month, accumulate a large merit score, and then coast indefinitely without contributing anything. Even worse, an attacker could spin up thousands of fake nodes, run them hard for a short period, and then use that stored merit to attack the network later. Decay prevents this. Merit erodes continuously without active participation, which means the only way to maintain standing is to genuinely keep participating. No one can stockpile merit as a weapon.

---

## How Valid Blockchain Resists Sybil Attacks

A Sybil attack is when a single person or organization creates many fake identities to gain disproportionate control over a network. This is the fundamental adversarial problem that every permissionless blockchain has to address.

Bitcoin and Ethereum address it through cost. Bitcoin makes it expensive in electricity and hardware. Ethereum makes it expensive in token value. Valid Blockchain addresses it through time and behavior, which are fundamentally different because they cannot be purchased.

The system works in layers.

The first layer is a 90-day observation period. Every new node enters an observer pool when it connects. Observer nodes can see the network and participate in gossip, but they cannot produce blocks. After 90 days of demonstrated honest behavior, a node becomes eligible for promotion to the validator pool. There is no way to skip this period. Time is the cost.

The second layer is the one-IP-per-identity rule. Two nodes sharing the same public IP address are treated as a single node. Someone trying to run ten fake validators from one machine gets one validator's worth of influence.

The third layer is heartbeat correlation detection. This is where the system gets genuinely novel. When blocks are broadcast across the network, every peer receives them at slightly different times based on physical distance, network routing, and hardware speed. Legitimate validators scattered across different locations and different hardware show natural variation in their response timing. Fake nodes running on the same server or the same data center receive broadcasts within microseconds of each other because they share the same network connection.

The system measures these timing patterns continuously during the 90-day observation window. A group of nodes that consistently respond to network events at nearly identical times, even with artificial randomness injected, produces a statistical signature that is distinguishable from genuine distributed behavior. Those nodes get flagged as co-located and receive a merit penalty that takes years of clean behavior to recover from.

The reason this cannot be beaten at scale is straightforward. An attacker can write a script to inject fake timing variance. But maintaining that convincingly across hundreds or thousands of nodes, for 90 days continuously, while also meeting the behavioral merit requirements for each individual node, while also making sure the coordination itself does not show up as a detectable pattern, is not a problem that automation can solve. The humans required to oversee that would be the bottleneck. That bottleneck is the point.

---

## How Chain State Bloat Is Defeated

Most blockchains get heavier as they get older. Every transaction adds to the state that full nodes have to store. Valid Blockchain solves this with a simple and elegant approach.

The entire chain state lives in memory during operation. Balances, transaction history, block data, all of it is held in RAM as plain data structures. No database. No disk writes during normal operation.

Every six hours, the oldest block data is written to a compact archive file and removed from memory. The node then holds only the last six hours of blocks in RAM, regardless of how long the chain has been running. A validator node that has been running for five years uses approximately the same memory as one that started yesterday.

Those archive files are also uploaded to Arweave, a permanent storage network, so the historical record of the chain exists independently of any single validator. When a new node joins, it syncs the current state from its peers and does not need to download years of history to start participating.

The hardware requirements for running a Valid Blockchain node do not grow with the network. That is a deliberate guarantee, not a current limitation that might change later.

---

## The VLid Token

VLid is the native token of Valid Blockchain. Understanding how it works starts with understanding what it is not.

There is no pre-mine. No tokens were created before the network launched and handed to the team, investors, or advisors. There is no venture capital allocation. No portion of the supply was sold privately to funds or early backers who now hold a financial advantage over everyone else. There is no team reserve. The person who built this (me) earn the same way everyone else does, by running validator nodes and producing blocks.

Every VLid that will ever exist is created by the network itself, one block at a time, as payment to the validator who produced that block. Tokens enter circulation only as a record of real work done. That is the entire issuance model.

The total supply is capped at 33 million VLid. It will never exceed that number. The cap is enforced in code, not by policy.

**How the emission schedule works**

The 33 million VLid is released over 21 years, divided into three periods of seven years each. The first period releases 60 percent of the total supply. The second period releases 30 percent. The third period releases the remaining 10 percent. Block rewards decay at the period boundaries, not gradually over time, which gives validators a clear and predictable schedule to plan around.

In the first epoch, the block reward is 0.0808 VLid per block. At one block every 10 seconds, that is roughly 504 blocks per hour and about 4.4 million blocks over seven years. The math works out so that 60 percent of the cap is distributed over that window at the fixed reward rate. When the first epoch ends, the reward rate drops to reflect the lower allocation for the second epoch, and so on.

Once the 33 million cap is reached, block rewards stop and validators earn entirely from transaction fees. This mirrors the long-term model of Bitcoin, where fee revenue is expected to sustain the network after the fixed supply is exhausted.

**What this means for validators**

Validators earn VLid every time they produce a block. The frequency with which any given validator produces blocks depends on their merit score relative to the rest of the validator pool. Higher merit increases the probability of being selected as the producer in any given slot.

This creates a direct and honest relationship between contribution and reward. A validator who participates reliably, maintains their node, and builds genuine standing in the network earns more. A validator who is inconsistent or misbehaves earns less and eventually loses selection priority to validators who have been more reliable.

There is no minimum VLid balance required to become a validator. There is no lockup period. There is no slashing of token balances for misbehavior. Misbehavior is punished through merit loss, not token confiscation. A validator who behaves badly loses standing and earns fewer rewards. Their tokens are never at risk from the protocol itself.

**Governance and ecosystem grants**

Governance of Valid Blockchain is merit-based. Voting weight comes from a combination of validator merit score and wallet age, not from how many tokens you hold. A validator who has been participating honestly for two years has more governance influence than a wallet that recently acquired a large token balance without any participation history. This is a deliberate choice to prevent the accumulation of governance power by wealthy token holders, which is the failure mode of every token-weighted governance system.

Ecosystem grants are being proposed as a formal governance mechanism. Developers and teams will be able to propose projects that benefit the Valid ecosystem, have the community vote on them using merit-weighted votes, and receive payment in VLid on a milestone basis rather than upfront. This is meant to fund the kind of work that makes the ecosystem stronger without any of the misalignment that comes from investor funding, where the funder's interests and the community's interests often diverge.

The specific distribution parameters for the genesis allocation and validator rewards will be finalized in v0.9.x based on what is learned during testnet. The principles are fixed. The exact numbers are not yet final.

---

## Testnet

The testnet is a six-month period before mainnet launch that serves two purposes.

The first is finding and fixing bugs under real conditions. Real participants, real hardware diversity, real geographic distribution, and real adversarial pressure from community members actively trying to break things. Every bug found during testnet is a bug that does not make it to mainnet.

The second is bootstrapping the initial validator merit set. A validator who participates consistently through the full testnet period arrives at mainnet with six months of verified behavioral history. That history is their standing in the network from day one. It is why early testnet participation is valuable beyond any token reward.

Testnet participants are rewarded from the genesis allocation based on their leaderboard standing. The leaderboard tracks attendance at live sessions, participation, bug reports, bug fixes, articles, social media posts, videos, and test completions. Points are awarded manually by the project team based on verified activity.

---

## Built to Be Forked

**Note:** Project is in a pre-released state and pre-audit. Audits will come ater testnet, bu before mainnet. 

Valid Blockchain is designed to be forked. This is not a footnote. It is a core design goal that shaped every architectural decision from the beginning.

The protocol branch of the repository is a clean, documented, fully auditable starting point for any team that wants to build their own blockchain on the TPI consensus foundation. The codebase is compact by design. The components are clearly separated. The configuration is simple and well-documented. A developer with a solid Rust background should be able to read the entire codebase, understand it, and start adapting it within days.

**The Linux model**

The relationship between Valid Blockchain and its forks is explicitly modeled on the relationship between the Linux kernel and its distributions. The kernel is maintained by a core team and provides a stable, tested foundation. Distributions make their own choices about what to include, how to configure the system, and who they are building for. Ubuntu is different from Arch Linux, which is different from Red Hat. All of them benefit when the kernel improves. All of them can contribute improvements back that benefit the whole ecosystem.

Valid Blockchain is the kernel. The public blockchain is one distribution. The private network branch is another, built for closed deployments where the validator set is known and trusted. Future forks by other teams are additional distributions.

Every successful fork validates the architecture. Every improvement to the architecture benefits every fork that has not diverged too far from the source. The ecosystem grows in ways that no single team could fund or build alone. This is exactly the dynamic that has made Linux successful across an extraordinary range of applications, from smartphones to supercomputers to satellites.

**Ecosystem grants**

Ecosystem grants are the mechanism by which this virtuous cycle gets funded. Tooling that makes it easier to fork and configure a Valid-based chain benefits every deployment in the ecosystem, not just the one that requested it. A library that improves the TPI consensus implementation benefits every fork that uses it. Grant recipients have obligations to the community that funded them rather than to investors with their own financial interests. The resulting software is more likely to serve what the community actually needs.

**Who should fork Valid Blockchain**

The use cases for Valid-derived chains are broad and concrete.

Hospital and healthcare networks need an auditable shared ledger that does not depend on any single vendor, does not expose patient data to external networks, and provides Byzantine fault tolerance among the trusted parties operating it. A Valid fork configured as a private network with hospital-operated validator nodes provides exactly this. Patient records, access logs, prescription data, and test results can be recorded with immutable timestamps. No public chain is involved. No tokens are required.

Warehouse and supply chain operations involve multiple parties who need a shared ledger they can all trust. Suppliers, warehouses, logistics companies, and retailers each have their own systems and their own incentives. A Valid fork with permissioned validator access for each party provides a neutral coordination layer where inventory movements are recorded as transactions and disputes are resolved by querying the chain rather than by trusting any single party's records.

Corporate and enterprise backends often need multi-party agreement on financial records, approval workflows, and audit trails without giving any single department unilateral authority to modify records. A Valid-derived private chain where each relevant department runs a validator node provides this. A finance department, a legal department, and an operations department running three validator nodes cannot collectively falsify records without all three agreeing, which means the records are trustworthy in a way that a single database controlled by one department is not.

Gaming companies need a fast, cheap, tamper-evident ledger for in-game economies. Players need to trust that the game operator cannot arbitrarily change their balances or fabricate transaction histories. A Valid fork configured for high-throughput private operation can handle in-game asset transfers, marketplace transactions, and achievement records with sub-second finality. The game operator controls the validator set and maintains operational control while providing players with an auditable, verifiable record.

Community organizations, cooperatives, credit unions, and neighborhood groups can run a Valid-derived chain for their specific community without any software licensing cost, without any cloud vendor dependency, and without any single member having unilateral control over the ledger. The 2GB RAM minimum makes this accessible to communities without significant technical resources. The TPI consensus means the ledger is controlled collectively by whoever operates the validator nodes, not by any one person.

Any team that wants to launch a custom public blockchain with TPI consensus but their own token economics, governance model, or application layer can fork from the protocol branch. They inherit a working, tested consensus mechanism and network layer and build their application-specific logic on top without starting from scratch.

---

## What Is Coming

The following projects are in active development or planned for release after mainnet launch.

**VNS (Valid Name Service)** is a naming layer that lets validators, addresses, and services register human-readable names on the network. Instead of sharing a long cryptographic address, a validator can register a name that others can use to find their node or send them tokens. VNS uses own-forever registration rather than recurring renewal, which means names cannot be lost because someone forgot to renew them on time. There is no separate token for VNS. Everything settles in VLid. VNS is in active development and will be released post-v1.0.

**VIPFS (Valid IPFS)** is a distributed storage layer for the Valid ecosystem. Files, websites, applications, media, and arbitrary data can be stored on VIPFS with cryptographic content addressing, meaning the address of a piece of content is derived from the content itself rather than from where it happens to be stored. This makes it impossible to serve different content under the same address without detection. VIPFS will eventually replace Arweave as the archive storage backend for the blockchain itself. The current architecture is already designed to make that transition seamless when the time comes. VIPFS is in active design and will be released post-v1.0.

**KEVIN (Distributed AI Inference)** is a compute marketplace where validators and users can contribute processing power for AI inference tasks and receive VLid in return. Someone who needs to run a large language model or an image generation model but does not have the hardware for it can pay VLid to access compute contributed by network participants. Someone with spare GPU capacity can earn VLid by making it available. There is no separate KEVIN token. Everything settles in VLid. KEVIN will be released after VNS and VIPFS have established the naming and storage foundation the marketplace needs. K.E.V.I.N. currently exists as a Discord bot with local AI integration and Valid Blockchain awareness. The production KEVIN network is the distributed evolution of that concept.

**Valid Browser** is a fork of the Brave browser with Valid Blockchain integrations built in at the browser level rather than added as extensions. The browser has a built-in L1 wallet, native resolution of VNS names so users can navigate to human-readable addresses in the URL bar, and native access to VIPFS content without any configuration. It supports direct payment signing and blockchain interactions without exposing private keys to web content. The initial scope is the Valid network integrations specifically. The browser will expand from there rather than attempting to replicate everything Brave does from the start.

**Valid Terminal** is a security-hardened terminal emulator forked from Alacritty. It includes filtering in both the keyboard input path and the paste path to catch homoglyph attacks, which are a class of attack where characters that look identical to normal letters are used to disguise malicious commands. For people running validator nodes and interacting regularly with a terminal, this is a meaningful security improvement over standard terminal emulators.

**Valid Vault** is a local password manager with fingerprint authentication and encrypted sync built for the Valid ecosystem. It stores credentials locally rather than in the cloud, uses hardware-based authentication where available, and synchronizes encrypted vaults across devices without any third party having access to the unencrypted contents.

---

## How to Get Involved

The testnet is the starting point. Join the Discord, run a node, show up to live development sessions, report bugs, write about the project, and help build something that belongs to the people running it.

The code is open and the architecture is highly documented. The whitepaper covers the full technical detail. All of it lives in the repository.

**Repository:** https://github.com/HiImRook/accessible-tpi-chain  
**Discord:** https://discord.gg/2SP383cJs9

---

*Copyright (c) 2024-2026 by Rook. MIT License.*
