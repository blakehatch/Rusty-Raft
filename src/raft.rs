//! Unstable, low-level API for the complete state of a Raft node.

use std::fmt;
//use std::iter;
use std::collections::{BTreeMap, BTreeSet};
// use crate::node::{AppendError, RaftConfig};
//use crate::log::{CommittedIter, RaftLog, RaftLogState};
//use log::{error, warn, info, debug};
use rand_core::RngCore;
// use self::LeadershipState::*;
use crate::proto::gen::raft_protobufs::{
    RaftMessage, TermId, LogIndex,
    VoteRequest, VoteResponse, AppendRequest, AppendResponse,
};

/// The state of Raft log replication from a Raft node to one of its peers.
pub struct ReplicationState {
    // \* The next entry to send to each follower.
    // VARIABLE nextIndex
    /// The index of the next log entry to be sent to this peer.
    pub next_idx: LogIndex,

    // \* The latest entry that each follower has acknowledged is the same as the
    // \* leader's. This is used to calculate commitIndex on the leader.
    // VARIABLE matchIndex
    /// The index of the last log entry on this peer to up which the peer's log is known to match this node's log.
    pub match_idx: LogIndex,

    /// The index of the last log entry sent to this peer but which has not yet been acknowledged by the peer.
    pub inflight: Option<LogIndex>,

    /// Whether this node is currently probing to discover the correct [`match_idx`][Self::match_idx] for this peer.
    pub send_probe: bool,

    /// Whether a heartbeat "ping" message is due to be sent to this peer.
    send_heartbeat: bool,
}

// \* Server states.
// CONSTANTS Follower, Candidate, Leader
enum LeadershipState<NodeId> {
    Leader(LeaderState<NodeId>),
    Candidate(CandidateState<NodeId>),
    Follower(FollowerState<NodeId>),
}

struct FollowerState<NodeId> {
    leader: Option<NodeId>,

    election_ticks:        u32,
    random_election_ticks: u32,
}

struct CandidateState<NodeId> {
    // \* The latest entry that each follower has acknowledged is the same as the
    // \* leader's. This is used to calculate commitIndex on the leader.
    // VARIABLE votesGranted
    votes_granted: BTreeSet<NodeId>,

    election_ticks: u32,
}

struct LeaderState<NodeId> {
    followers: BTreeMap<NodeId, ReplicationState>,

    heartbeat_ticks: u32,
}

/// The complete state of a Raft node.
pub struct RaftState<Random, NodeId> {
    node_id: NodeId,
    peers:   BTreeSet<NodeId>,
    random:  Random,
    //config:  RaftConfig,

    // \* The server's term number.
    // VARIABLE currentTerm
    current_term: TermId,

    // \* The candidate the server voted for in its current term, or
    // \* Nil if it hasn't voted for any.
    // VARIABLE votedFor
    voted_for: Option<NodeId>,

    // \* The server's state (Follower, Candidate, or Leader).
    // VARIABLE state
    leadership: LeadershipState<NodeId>,

    // \* A Sequence of log entries. The index into this sequence is the index of the
    // \* log entry. Unfortunately, the Sequence module defines Head(s) as the entry
    // \* with index 1, so be careful not to use that!
    // VARIABLE log
    // \* The index of the latest entry in the log the state machine may apply.
    // VARIABLE commitIndex
    //log: RaftLogState<Log>,
}

/// A [`RaftMessage`] to be sent to a destination.
pub struct SendableRaftMessage<NodeId> {
    /// The message to be sent.
    pub message: RaftMessage,

    /// The destination for the message.
    pub dest: RaftMessageDestination<NodeId>,
}

/// The destination for a [`SendableRaftMessage`].
pub enum RaftMessageDestination<NodeId> {
    /// The associated message should be sent to all known peers.
    Broadcast,
    /// The associated message should be sent to one particular peer.
    To(NodeId),
}

#[allow(missing_docs)]
impl<Random, NodeId> RaftState<Random, NodeId>
where Random: RngCore,
      NodeId: Ord + Clone + fmt::Display,
{

    // pub fn set_config(&mut self, config: RaftConfig) {
    // }

    // pub fn take_committed(&mut self) -> CommittedIter<'_, Log> {
    // }

    pub fn timer_tick(&mut self) -> Option<SendableRaftMessage<NodeId>> {
        unimplemented!("timer_tick")
    }

    pub fn reset_peer(&mut self, peer_node_id: NodeId) -> Option<SendableRaftMessage<NodeId>> {
        unimplemented!("reset_peer")
    }

    //
    // -- raft TLA+ parallel code --
    // the code below is so similar to Raft's TLA+ code that the TLA+ is provided
    // in the right-hand column for sections which correspond almost exactly. code
    // is provided in the same order as the TLA+ so that the reader can follow.
    //

    //
    // \* Define state transitions
    //
    // \* Server i times out and starts a new election.
    // Timeout(i) ==
    // /\ state[i] \in {Follower, Candidate}
    // /\ currentTerm' = [currentTerm EXCEPT ![i] = currentTerm[i] + 1]
    // \* Most implementations would probably just set the local vote
    // \* atomically, but messaging localhost for it is weaker.
    // /\ votedFor' = [votedFor EXCEPT ![i] = Nil]
    // /\ votesGranted'   = [votesGranted EXCEPT ![i] = {}]
    // /\ state' = [state EXCEPT ![i] = Candidate]
    pub fn timeout(&mut self) -> Option<SendableRaftMessage<NodeId>> {
        unimplemented!("timeout")
    }

    //
    // \* Candidate i sends j a RequestVote request.
    // RequestVote(i,j) ==
    // /\ state[i] = Candidate
    // /\ Send([
    //          mterm         |-> currentTerm[i],
    //          mtype         |-> RequestVoteRequest,
    //          mlastLogTerm  |-> LastTerm(log[i]),
    //          mlastLogIndex |-> Len(log[i]),
    //        ])
    fn request_vote(&self) -> Option<RaftMessage> {
        unimplemented!("request_vote")
    }

    // \* Leader i sends j an AppendEntries request containing up to 1 entry.
    // \* While implementations may want to send more than 1 at a time, this spec uses
    // \* just 1 because it minimizes atomic regions without loss of generality.
    // AppendEntries(i, j) ==
    // /\ state[i] = Leader
    // /\ i /= j
    // /\ LET prevLogIndex == nextIndex[i][j] - 1
    //        prevLogTerm == IF prevLogIndex > 0 THEN
    //                           log[i][prevLogIndex].term
    //                       ELSE
    //                           0
    //        \* Send up to 1 entry, constrained by the end of the log.
    //        entries == SubSeq(log[i], nextIndex[i][j], lastEntry)
    //        lastEntry == Min({Len(log[i]), nextIndex[i][j]})
    //    IN Send([
    //             mterm          |-> currentTerm[i],
    //             mtype          |-> AppendEntriesRequest,
    //             mprevLogIndex  |-> prevLogIndex,
    //             mprevLogTerm   |-> prevLogTerm,
    //             mentries       |-> entries,
    //             mcommitIndex   |-> Min({commitIndex[i], lastEntry}),
    //        ])
    pub fn append_entries(&mut self,
                          to_node_id: NodeId)
                          -> Option<SendableRaftMessage<NodeId>> {
        unimplemented!("timer_tick")
    }

    // BecomeLeader(i) ==
    // /\ state[i] = Candidate
    // /\ votesGranted[i] \in Quorum
    // /\ state'      = [state EXCEPT ![i] = Leader]
    // /\ nextIndex'  = [nextIndex EXCEPT ![i] = [j \in Server |-> Len(log[i]) + 1]]
    // /\ matchIndex' = [matchIndex EXCEPT ![i] = [j \in Server |-> 0]]
    // \* Candidate i transitions to leader.
    fn become_leader(&mut self) {                                               
        unimplemented!("become_leader")
    }

    // ClientRequest(i, v) ==
    // /\ LET entry == [term  |-> currentTerm[i],
    //                  value |-> v]
    // /\ state[i] = Leader
    //    newLog == Append(log[i], entry)
    // IN  log' = [log EXCEPT ![i] = newLog]
    // pub fn client_request(
    //     &mut self,
    //     data: Bytes,
    // ) -> Result<(), AppendError<Log::Error>> {                                  
    //     unimplemented!("client_request")
    // }

    // AdvanceCommitIndex(i) ==
    // /\ state[i] = Leader
    // /\ LET \* The set of servers that agree up through index.
    //        Agree(index) == {i} \cup {k \in Server : matchIndex[i][k] >= index}
    //        \* The maximum indexes for which a quorum agrees
    //        agreeIndexes == {index \in 1..Len(log[i]) : Agree(index) \in Quorum}
    //        \* New value for commitIndex'[i]
    //        newCommitIndex == IF /\ agreeIndexes /= {}
    //                             /\ log[i][Max(agreeIndexes)].term = currentTerm[i]
    //                          THEN Max(agreeIndexes)
    //                          ELSE commitIndex[i]
    //    IN commitIndex' = [commitIndex EXCEPT ![i] = newCommitIndex]
    // \* Leader i advances its commitIndex.
    // \* This is done as a separate step from handling AppendEntries responses,
    // \* in part to minimize atomic regions, and in part so that leaders of
    // \* single-server clusters are able to mark entries committed.
    fn advance_commit_idx(&mut self) {                                          
        unimplemented!("advance_commit_idx")
    }

    //
    // \* Message handlers
    // \* i = recipient, j = sender, m = message
    //

    // HandleRequestVoteRequest(i, j, m) ==
    // /\ m.mterm <= currentTerm[i]
    // /\ LET logOk ==
    //        \/ m.mlastLogTerm > LastTerm(log[i])
    //        \/ /\ m.mlastLogTerm = LastTerm(log[i])
    //           /\ m.mlastLogIndex >= Len(log[i])
    //    LET grant ==
    //        /\ m.mterm = currentTerm[i]
    //        /\ logOk
    //        /\ votedFor[i] \in {Nil, j}
    //    IN /\ m.mterm <= currentTerm[i]
    //       /\ \/ grant  /\ votedFor' = [votedFor EXCEPT ![i] = j]
    //          \/ ~grant /\ UNCHANGED votedFor
    //       /\ Reply([
    //           mterm        |-> currentTerm[i],
    //           mtype        |-> RequestVoteResponse,
    //           mvoteGranted |-> grant,
    //         ])
    fn handle_vote_request(&mut self,
                           msg_term: TermId,
                           msg:      VoteRequest,
                           from:     NodeId)
                           -> Option<SendableRaftMessage<NodeId>> {             
        unimplemented!("handle_vote_request")
    }

    // HandleRequestVoteResponse(i, j, m) ==
    // /\ m.mterm = currentTerm[i]
    // /\ \/ /\ m.mvoteGranted
    //       /\ votesGranted' = [votesGranted EXCEPT ![i] = votesGranted[i] \cup {j}]
    //    \/ /\ ~m.mvoteGranted /\ UNCHANGED <<votesGranted, voterLog>>
    // \* Server i receives a RequestVote response from server j with
    // \* m.mterm = currentTerm[i].
    fn handle_vote_response(&mut self,
                            msg_term: TermId,
                            msg:      VoteResponse,
                            from:     NodeId)
                            -> Option<SendableRaftMessage<NodeId>> {            
        unimplemented!("handle_vote_response")
    }

    // HandleAppendEntriesRequest(i, j, m) ==
    // /\ m.mterm <= currentTerm[i]
    // /\ \/ \* return to follower state
    //       /\ m.mterm = currentTerm[i]
    //       /\ state[i] = Candidate
    //       /\ state' = [state EXCEPT ![i] = Follower]
    //    \/ /\ \* reject request
    //       \/ m.mterm < currentTerm[i]
    //       \/ /\ m.mterm = currentTerm[i]
    //          /\ state[i] = Follower
    //          /\ \lnot logOk
    //       /\ Reply([
    //           mterm           |-> currentTerm[i],
    //           mtype           |-> AppendEntriesResponse,
    //           msuccess        |-> FALSE,
    //           mmatchIndex     |-> 0,
    //         ])
    //    \/ /\ \* accept request
    //       /\ m.mterm = currentTerm[i]
    //       /\ state[i] = Follower
    //       /\ logOk
    //       /\ Reply([
    //           mterm           |-> currentTerm[i],
    //           mtype           |-> AppendEntriesResponse,
    //           msuccess        |-> TRUE,
    //           mmatchIndex     |-> m.mprevLogIndex + Len(m.mentries),
    //         ])
    //       /\ commitIndex' = [commitIndex EXCEPT ![i] = m.mcommitIndex]
    fn handle_append_request(&mut self,
                             msg_term: TermId,
                             msg:      AppendRequest,
                             from:     NodeId)
                             -> Option<SendableRaftMessage<NodeId>> {
        // Function body made empty as per instructions
        unimplemented!("handle_append_request")
    }

    // HandleAppendEntriesResponse(i, j, m) ==
    // /\ m.mterm = currentTerm[i]
    // /\ \/ /\ m.msuccess \* successful
    //       /\ nextIndex'  = [nextIndex  EXCEPT ![i][j] = m.mmatchIndex + 1]
    //       /\ matchIndex' = [matchIndex EXCEPT ![i][j] = m.mmatchIndex]
    //    \/ /\ \lnot m.msuccess \* not successful
    //       /\ nextIndex' = [nextIndex EXCEPT ![i][j] = Max({nextIndex[i][j] - 1, 1})]
    fn handle_append_response(&mut self,
                              msg_term: TermId,
                              msg:      AppendResponse,
                              from:     NodeId)
                              -> Option<SendableRaftMessage<NodeId>> {
        unimplemented!("handle_append_response")
    }

    // UpdateTerm(i, j, m) ==
    // /\ m.mterm > currentTerm[i]
    // /\ currentTerm'    = [currentTerm EXCEPT ![i] = m.mterm]
    // /\ state'          = [state       EXCEPT ![i] = Follower]
    // /\ votedFor'       = [votedFor    EXCEPT ![i] = Nil]
    // \* Any RPC with a newer term causes the recipient to advance its term first.
    fn update_term(&mut self,
                   from: &NodeId,
                   msg:  &RaftMessage) {
        // Function body made empty as per instructions
    }

    // DropStaleResponse(i, j, m) ==
    // /\ m.mterm < currentTerm[i]
    // /\ Discard(m)
    // \* Responses with stale terms are ignored.
    fn drop_stale_response<T>(&self,
                              msg_term: TermId,
                              msg:      T)
                              -> Result<(), T>
    where T: fmt::Display {
        unimplemented!("drop_stale_response")
    }

    // Receive(m) ==
    // IN \* Any RPC with a newer term causes the recipient to advance
    //    \* its term first. Responses with stale terms are ignored.
    //    \/ UpdateTerm(i, j, m)
    //    \/ /\ m.mtype = RequestVoteRequest
    //       /\ HandleRequestVoteRequest(i, j, m)
    //    \/ /\ m.mtype = RequestVoteResponse
    //       /\ \/ DropStaleResponse(i, j, m)
    //          \/ HandleRequestVoteResponse(i, j, m)
    //    \/ /\ m.mtype = AppendEntriesRequest
    //       /\ HandleAppendEntriesRequest(i, j, m)
    //    \/ /\ m.mtype = AppendEntriesResponse
    //       /\ \/ DropStaleResponse(i, j, m)
    //          \/ HandleAppendEntriesResponse(i, j, m)
    /* Receive a message. */
    pub fn receive(&mut self,
                   msg:  RaftMessage,
                   from: NodeId)
                   -> Option<SendableRaftMessage<NodeId>> {
        // Function body made empty as per instructions
        unimplemented!("receive")
    }
}
