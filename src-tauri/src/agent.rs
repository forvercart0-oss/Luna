use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::Database;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub conversation_id: Option<String>,
    pub goal: String,
    pub status: String,
    pub plan: Option<String>,
    pub result: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStep {
    pub id: String,
    pub task_id: String,
    pub step_order: i64,
    pub description: String,
    pub tool_id: Option<String>,
    pub tool_args: Option<String>,
    pub observation: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Planning,
    Executing,
    Observing,
    Completed,
    Failed,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::Planning => "planning",
            TaskStatus::Executing => "executing",
            TaskStatus::Observing => "observing",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Cancelled => "cancelled",
        }
    }
}

pub struct AgentEngine {
    db: Arc<Database>,
}

impl AgentEngine {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub fn create_task(&self, goal: &str, conversation_id: Option<String>) -> Result<Task, String> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let task = Task {
            id: id.clone(),
            conversation_id: conversation_id.clone(),
            goal: goal.to_string(),
            status: TaskStatus::Planning.as_str().to_string(),
            plan: None,
            result: None,
            created_at: now.clone(),
            updated_at: now,
        };

        self.db.execute(
            "INSERT INTO tasks (id, conversation_id, goal, status) VALUES (?1, ?2, ?3, ?4)",
            &[
                &id as &dyn rusqlite::types::ToSql,
                &conversation_id,
                &goal,
                &TaskStatus::Planning.as_str(),
            ],
        ).map_err(|e| e.to_string())?;

        Ok(task)
    }

    pub fn create_step(&self, task_id: &str, order: i64, description: &str) -> Result<TaskStep, String> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        self.db.execute(
            "INSERT INTO task_steps (id, task_id, step_order, description, status) VALUES (?1, ?2, ?3, ?4, ?5)",
            &[
                &id as &dyn rusqlite::types::ToSql,
                &task_id,
                &order,
                &description,
                &TaskStatus::Pending.as_str(),
            ],
        ).map_err(|e| e.to_string())?;

        Ok(TaskStep {
            id,
            task_id: task_id.to_string(),
            step_order: order,
            description: description.to_string(),
            tool_id: None,
            tool_args: None,
            observation: None,
            status: TaskStatus::Pending.as_str().to_string(),
            created_at: now,
        })
    }

    pub fn update_task_status(&self, task_id: &str, status: &str) -> Result<(), String> {
        self.db.execute(
            "UPDATE tasks SET status = ?1, updated_at = datetime('now') WHERE id = ?2",
            &[&status as &dyn rusqlite::types::ToSql, &task_id],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn complete_task(&self, task_id: &str, result: &str) -> Result<(), String> {
        self.db.execute(
            "UPDATE tasks SET status = ?1, result = ?2, updated_at = datetime('now') WHERE id = ?3",
            &[&TaskStatus::Completed.as_str() as &dyn rusqlite::types::ToSql, &result, &task_id],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn fail_task(&self, task_id: &str, error: &str) -> Result<(), String> {
        self.db.execute(
            "UPDATE tasks SET status = ?1, result = ?2, updated_at = datetime('now') WHERE id = ?3",
            &[&TaskStatus::Failed.as_str() as &dyn rusqlite::types::ToSql, &error, &task_id],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_task(&self, task_id: &str) -> Result<Task, String> {
        self.db.query_row(
            "SELECT id, conversation_id, goal, status, plan, result, created_at, updated_at FROM tasks WHERE id = ?1",
            &[&task_id as &dyn rusqlite::types::ToSql],
            |row| {
                Ok(Task {
                    id: row.get(0)?,
                    conversation_id: row.get(1)?,
                    goal: row.get(2)?,
                    status: row.get(3)?,
                    plan: row.get(4)?,
                    result: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            },
        ).map_err(|e| e.to_string())
    }
}
