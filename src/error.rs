use std;

#[derive(Debug)]
pub struct KError { pub file: &'static str, pub line: u32 }

impl std::error::Error for KError {
    fn description (&self) -> &str { "just an error" }
}

impl std::fmt::Display for KError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "KErr: ")
    }
}

pub type KResult<T> =  Result<T, KError>;

