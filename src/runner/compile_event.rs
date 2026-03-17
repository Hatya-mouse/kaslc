pub(super) enum CompileEvent {
    Parsing,
    Building,
    Error(String),
}
