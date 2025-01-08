use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub status: String,
    pub starts_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
    pub starts_at: i64,
    pub ends_at: Option<i64>,
}

impl CreateTaskRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Title cannot be empty.".to_string());
        }

        if self.description.trim().is_empty() {
            return Err("Description cannot be empty.".to_string());
        }

        if Utc.timestamp_opt(self.starts_at, 0).single().is_none() {
            return Err("Invalid date timestamp provided.".to_string());
        }

        if let Some(ends_at) = self.ends_at {
            if Utc.timestamp_opt(ends_at, 0).single().is_none() {
                return Err("Invalid end date timestamp provided.".to_string());
            }

            if ends_at <= self.starts_at {
                return Err("End date must be later than start date.".to_string());
            }
        }

        Ok(())
    }

    // TODO: вынести куда-нибудь
    fn datetime(timestamp: i64) -> DateTime<Utc> {
        Utc.timestamp_opt(timestamp, 0)
            .single()
            .unwrap_or_else(|| Utc::now())
    }
}

impl From<CreateTaskRequest> for Task {
    fn from(request: CreateTaskRequest) -> Self {
        let starts_at = CreateTaskRequest::datetime(request.starts_at);
        let ends_at = request.ends_at.map(CreateTaskRequest::datetime);

        Task {
            id: 0, // OK
            title: request.title,
            description: request.description,
            // TODO: временный хардкод
            status: "pending".to_string(),
            starts_at,
            ends_at,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct TaskResponse {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub status: String,
    pub starts_at: i64,
    pub ends_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        // let ends_at = match task.ends_at {
        //     Some(ends_at) => Some(ends_at.timestamp()),
        //     None => None,
        // };
        Self {
            id: task.id,
            title: task.title,
            description: task.description,
            status: task.status,
            starts_at: task.starts_at.timestamp(),
            ends_at: Some(100),
            // ends_at,
            created_at: task.created_at.timestamp(),
            updated_at: task.updated_at.timestamp(),
        }
    }
}
impl TaskResponse {
    pub fn from_tasks(tasks: Vec<Task>) -> Vec<Self> {
        tasks.into_iter().map(TaskResponse::from).collect()
    }
}
