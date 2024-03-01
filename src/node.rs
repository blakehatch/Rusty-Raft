use std::{io::Error, collections::BTreeSet};
use crate::raft::RaftState;
use core::fmt::Display;
use rand_core::RngCore;

// TODO: Add logs to this later
pub struct Node<Random, NodeId> {
    state: RaftState<Random, NodeId>,
}

/// Configurable parameters of a Raft node.
#[derive(Clone, Eq, PartialEq)]
pub struct RaftConfig {
    /// The minimum number of timer ticks between leadership elections.
    pub election_timeout_ticks: u32,

    /// The number of timer ticks between sending heartbeats to peers.
    pub heartbeat_interval_ticks: u32,

    /// The maximum number of bytes to replicate to a peer at a time.
    pub replication_chunk_size: usize,
}

impl<Random, NodeId> Node<Random, NodeId>
where Random: RngCore,
      NodeId: Ord + Clone + Display,
{
    pub fn new(
        id: NodeId,
        // TODO: Figure out if BTreeSet is a good data structure for async
        peers: BTreeSet<NodeId>,
        random: Random,
        config: RaftConfig,
    ) -> Self {
        Self {
            state: RaftState::new(
                id,
                peers,
                random,
                config
            )
        }

    }
}
