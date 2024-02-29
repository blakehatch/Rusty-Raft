use super::RaftLog;

pub struct RaftLogMemory {
    entries:       VecDeque<LogEntry>,
    prev_log_idx:  LogIndex,
    prev_log_term: TermId,
    last_taken:    LogIndex,
    data_len:      usize,
    data_capacity: usize,
}

pub struct Config {
    pub initial_entries_capacity: usize,
    pub data_capacity: usize,
}

impl RaftLogMemory {
    /// Constructs a new Raft log based on the provided configuration.
    pub fn new(config: Config) -> Self {
        Self {
            entries: VecDeque::with_capacity(config.initial_entries_capacity),
            prev_log_idx: LogIndex::default(),
            prev_log_term: TermId::default(),
            last_taken: LogIndex::default(),
            data_len: 0,
            data_capacity: config.data_capacity,
        }
    }
}

impl RaftLog for RaftLogMemory {

}