pub enum CommandResult {
    Added(String, String),
    Deleted(String, String),
    Renamed(String, String),
    Jump(String),
    Migrated(String),
    DisplayNone,
    Ok,
}
