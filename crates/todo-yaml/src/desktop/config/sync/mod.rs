pub mod from_str;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Sync {
    NoSync,
    GitPushPull {
        remote: String,
        branch: String,
        force: bool,
    },
}
