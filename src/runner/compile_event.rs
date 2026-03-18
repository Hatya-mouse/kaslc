use kasl::error::ErrorRecord;
use std::time::Duration;

pub(super) enum CompileEvent {
    Parsing,
    Building,
    Builded(Duration),
    Running,
    Finished {
        exec_elapsed: Duration,
        max_elapsed: Duration,
        min_elapsed: Duration,
        avg_elapsed: Duration,
    },
    Error(String),
    KaslError(Vec<ErrorRecord>, String),
}
