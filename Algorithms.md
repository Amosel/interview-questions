## Distributed system

- Reed-Solomon error correction (Scalability)
- 2D parity check (Scalability)
- Distributed Hash Tables (improve latency)
- Paxos, Raft (Consistency)
- Vector Clocks (Partition Tolerance)
  - Vector Clocks and Version Vectors and Hybrid Logical Clocks are arrays or hash tables (in case the number of nodes in the network is dynamic) of integers, representing counters of events by node. They are used to capture causality and determining the relative order of events to prevent ambiguity in how state is calculated. This supports in Conflict-Free Replicated Data Types, and allow the merging/converging/reconciling state as derived from the ordering of events as known to the node at a time.
- Merkle Trees (Partition Tolerance)