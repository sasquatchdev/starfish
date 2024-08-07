#[derive(Debug, Clone)]
pub enum Flag {
    None,
    Suppressed,

    Information(String),
    Warning(String),
    Error(String),

    Fatal(String),
}
