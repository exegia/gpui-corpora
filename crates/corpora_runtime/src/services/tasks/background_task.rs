#[allow(unused_imports)]
use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundTask {
    pub id: TaskId,
    pub label: String,
    pub status: TaskStatus,
    pub progress: TaskProgress,
    pub started_at: AppTimestamp,
    pub finished_at: Option<AppTimestamp>,
    pub error: Option<String>,
}
