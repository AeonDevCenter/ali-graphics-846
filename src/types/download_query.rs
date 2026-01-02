use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DownloadQuery {
    name: String
}

impl DownloadQuery {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
