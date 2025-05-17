use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CompileCommand {
    pub directory: String,
    pub command: String,
    pub file: String,
}
