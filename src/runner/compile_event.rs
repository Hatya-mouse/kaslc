use std::time::Duration;

pub(super) enum CompileEvent {
    Parsing,
    Building,
    Builded(Duration),
    Error(String),
}
