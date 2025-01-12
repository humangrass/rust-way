use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Canceled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Completed => "completed",
            TaskStatus::Canceled => "canceled",
        }
    }

    pub fn from_str(status: &str) -> Result<Self, String> {
        match status {
            "pending" => Ok(TaskStatus::Pending),
            "in_progress" => Ok(TaskStatus::InProgress),
            "completed" => Ok(TaskStatus::Completed),
            "canceled" => Ok(TaskStatus::Canceled),
            _ => Err(format!("Unknown task status: {}", status)),
        }
    }
}

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub starts_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TaskModel> for Task {
    fn from(task: TaskModel) -> Self {
        let status = TaskStatus::from_str(&task.status).unwrap_or(TaskStatus::Pending);
        Self {
            id: task.id,
            title: task.title,
            description: task.description,
            status,
            starts_at: task.starts_at,
            ends_at: task.ends_at,
            created_at: task.created_at,
            updated_at: task.updated_at,
        }
    }
}

impl From<Task> for TaskModel {
    fn from(business_task: Task) -> Self {
        Self {
            id: business_task.id,
            title: business_task.title,
            description: business_task.description,
            status: business_task.status.as_str().to_string(),
            starts_at: business_task.starts_at,
            ends_at: business_task.ends_at,
            created_at: business_task.created_at,
            updated_at: business_task.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskModel {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub status: String,
    pub starts_at: DateTime<Utc>,
    pub ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct TaskRequest {
    pub title: String,
    pub description: String,
    pub status: String,
    pub starts_at: i64,
    pub ends_at: Option<i64>,
}

impl TaskRequest {
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

        if TaskStatus::from_str(&self.status).is_err() {
            return Err(format!("Invalid status: {}.", self.status));
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

impl From<TaskRequest> for Task {
    fn from(request: TaskRequest) -> Self {
        let starts_at = TaskRequest::datetime(request.starts_at);
        let ends_at = request.ends_at.map(TaskRequest::datetime);
        let status = TaskStatus::from_str(&request.status).unwrap_or(TaskStatus::Pending);

        Self {
            id: 0, // OK
            title: request.title,
            description: request.description,
            status,
            starts_at,
            ends_at,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct TaskResponse {
    pub id: i32,
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
        Self {
            id: task.id,
            title: task.title,
            description: task.description,
            status: task.status.as_str().to_string(),
            starts_at: task.starts_at.timestamp(),
            ends_at: task.ends_at.map(|e| e.timestamp()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_task_status_from_str() {
        assert_eq!(TaskStatus::from_str("pending").unwrap(), TaskStatus::Pending);
        assert_eq!(TaskStatus::from_str("in_progress").unwrap(), TaskStatus::InProgress);
        assert_eq!(TaskStatus::from_str("completed").unwrap(), TaskStatus::Completed);
        assert_eq!(TaskStatus::from_str("canceled").unwrap(), TaskStatus::Canceled);

        assert!(TaskStatus::from_str("unknown").is_err());
    }

    #[test]
    fn test_task_status_as_str() {
        assert_eq!(TaskStatus::Pending.as_str(), "pending");
        assert_eq!(TaskStatus::InProgress.as_str(), "in_progress");
        assert_eq!(TaskStatus::Completed.as_str(), "completed");
        assert_eq!(TaskStatus::Canceled.as_str(), "canceled");
    }

    #[test]
    fn test_task_request_validation_success() {
        let request = TaskRequest {
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            status: "pending".to_string(),
            starts_at: Utc::now().timestamp(),
            ends_at: None,
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_task_request_validation_empty_title() {
        let request = TaskRequest {
            title: "".to_string(),
            description: "Test Description".to_string(),
            status: "pending".to_string(),
            starts_at: Utc::now().timestamp(),
            ends_at: None,
        };

        assert_eq!(request.validate(), Err("Title cannot be empty.".to_string()));
    }

    #[test]
    fn test_task_request_validation_invalid_status() {
        let request = TaskRequest {
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            status: "invalid_status".to_string(),
            starts_at: Utc::now().timestamp(),
            ends_at: None,
        };

        assert_eq!(
            request.validate(),
            Err("Invalid status: invalid_status.".to_string())
        );
    }

    #[test]
    fn test_task_request_validation_negative_timestamps() {
        let request = TaskRequest {
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            status: "pending".to_string(),
            starts_at: -1,
            ends_at: None,
        };
        // It works!
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_task_request_to_task() {
        let request = TaskRequest {
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            status: "pending".to_string(),
            starts_at: Utc::now().timestamp(),
            ends_at: Some(Utc::now().timestamp() + 3600),
        };

        let task: Task = request.into();

        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, "Test Description");
        assert_eq!(task.status, TaskStatus::Pending);
    }

    #[test]
    fn test_task_to_task_response() {
        let task = Task {
            id: 1,
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            status: TaskStatus::Completed,
            starts_at: Utc::now(),
            ends_at: Some(Utc::now() + chrono::Duration::hours(1)),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let response: TaskResponse = task.into();

        assert_eq!(response.title, "Test Task");
        assert_eq!(response.status, "completed");
    }

    #[test]
    fn test_task_model_to_task_conversion() {
        let model = TaskModel {
            id: 1,
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            status: "in_progress".to_string(),
            starts_at: Utc::now(),
            ends_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let task: Task = model.into();

        assert_eq!(task.title, "Test Task");
        assert_eq!(task.status, TaskStatus::InProgress);
    }

    #[test]
    fn test_task_to_task_model_conversion() {
        let task = Task {
            id: 1,
            title: "Test Task".to_string(),
            description: "Test Description".to_string(),
            status: TaskStatus::Canceled,
            starts_at: Utc::now(),
            ends_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let model: TaskModel = task.into();

        assert_eq!(model.title, "Test Task");
        assert_eq!(model.status, "canceled");
    }
}
