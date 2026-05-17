# Relm
Distributed Personal Computing

## 1. Overview

Relm is a **personal distributed operating system** that treats a user’s devices (laptop, desktop, phone, home servers, and optional cloud nodes) as a single unified computational environment.

Relm is not a network system or cluster orchestrator.

It is an **operating system whose execution substrate spans multiple machines**.

---

## 2. Core Philosophy

### 2.1 OS-first abstraction
Relm defines computation, identity, and execution semantics independently of networking.

### 2.2 Personal computational fabric
A user does not operate multiple devices, rather they operate a single distributed system.

Devices are transient execution substrates, not system boundaries.

### 2.3 Continuity over locality
Relm prioritizes:

- continuity of computation  
- migration of workloads  
- persistence of state  
- policy-driven placement  

over static machine assignment.

### 2.4 Transport agnosticism
Relm must function regardless of underlying networking:

- LAN  
- WAN  
- NAT  
- VPN overlays  
- future IPv6-native meshes  

Networking is a dependency, not a design constraint.

---

## 3. System Architecture

Relm is structured into three conceptual layers:

- Identity Layer  
- Control Layer
- Execution Layer

---

## 4. Identity Layer

Identity is fully owned by Relm and is the foundational primitive of the system.

### 4.1 Cryptographic identity model

- User identity = root cryptographic key  
- Devices = signed key pairs derived from root  
- Workloads = signed portable execution entities  

Identity is not tied to IP addresses, hostnames, or network topology.

---

### 4.2 Identity hierarchy
User Root Key
|- Device Key (laptop)
|- Device Key (desktop)
|- Device Key (phone)
|- Cloud / remote nodes

Workload Identity (portable computational entity)
|- state
|- policy
|- execution constraints


Key property:

> Workloads are stable identities independent of where they execute.

---

## 6. Execution Model

### 6.1 Workload

A workload is a first-class Relm entity with:

- identity
- state
- execution constraints
- placement policy
- migration capability

Example constraints:

- GPU required
- low-latency preference
- trusted devices only
- local-only execution
- checkpoint interval requirements

---

### 6.2 Execution Plane (per-device agent)

Each device runs a Relm agent responsible for:

- executing workloads locally
- managing resources
- checkpointing state
- reporting telemetry
- enforcing constraints

Devices are execution hosts, not autonomous systems.

---

### 6.3 Control Plane (distributed coordination layer)

The control plane handles:

- workload scheduling decisions
- migration decisions
- policy enforcement
- reacting to node availability changes

It is:

- decentralized or eventually consistent
- not a single global controller
- not dependent on any fixed server

---

## 7. Continuity Model

Relm assumes computation is fragile and migratable.

Nodes may:

- disconnect at any time
- rejoin later
- move networks
- become temporarily unreachable

Therefore:

- workloads must be checkpointable
- execution must be resumable
- state must be recoverable
- computation must tolerate interruption

Continuity is a first-class semantic, not an implementation detail.

---

## 8. Connectivity Model

Relm assumes:

- partial connectivity
- non-uniform reachability
- intermittent availability
- opportunistic routing

Key principle:

> membership in Relm is independent of reachability

A device may belong to Relm but be offline or unreachable.

---

## 9. Discovery Model

Relm does not require global discovery.

Instead it uses:

- local LAN discovery (mDNS-style)
- manual or cryptographic bootstrap pairing
- peer-to-peer gossip propagation
- opportunistic reconnection

Discovery is a consequence of connectivity, not a prerequisite.

---

## 10. Trust and Policy Model

Relm uses capability-based trust:

- devices have roles (GPU node, mobile node, trusted node, etc.)
- workloads define execution constraints
- authorization is cryptographically enforced

Trust is:

- explicit
- user-controlled
- decentralized

---

## 11. Failure Model

Relm assumes failure is normal:

- nodes fail
- networks partition
- workloads pause
- state becomes temporarily inconsistent

System response:

- retryable execution
- checkpointing
- eventual convergence
- graceful degradation

---

## 12. Non-Goals

Relm explicitly does NOT attempt to:

- replace networking protocols
- solve NAT traversal
- guarantee global consensus
- assume stable cluster infrastructure
- depend on a central control plane
- enforce strict real-time consistency

---

## 13. Use of Existing Infrastructure

Relm intentionally builds on existing systems:

- WireGuard-based tunnels
- Tailscale (temporary experimental substrate)
- VPN overlays
- future IPv6-native networks

Infrastructure is replaceable plumbing.

---

## 14. Core Insight

Relm shifts computing from:

> computing on machines

to:

> computing across a personal, distributed, migratable fabric

Networking is incidental. Identity and computation are fundamental.

---

## 15. One-line Definition

Relm is a distributed operating system that treats a user’s devices as a single, policy-driven, migratable computational substrate, independent of underlying network topology.