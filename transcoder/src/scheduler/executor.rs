use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum JobStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

impl Display for JobStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Pending => write!(f, "pending"),
            JobStatus::Running => write!(f, "running"),
            JobStatus::Completed => write!(f, "completed"),
            JobStatus::Failed => write!(f, "failed"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JobExecution {
    pub id: Uuid,
    pub job_name: String,
    pub status: JobStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub duration_seconds: Option<i32>,
}

pub struct JobExecutor {
    pool: std::sync::Arc<PgPool>,
}

impl JobExecutor {
    pub fn new(pool: std::sync::Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Record job start  
    pub async fn start_job(&self, job_name: &str) -> Result<Uuid, sqlx::Error> {
        let job_id = Uuid::new_v4();

        sqlx::query(
            r#"INSERT INTO job_executions (id, job_name, status, started_at)  
               VALUES ($1, $2, $3, NOW())"#,
        )
        .bind(job_id)
        .bind(job_name)
        .bind("running")
        .execute(self.pool.as_ref())
        .await?;

        println!("▶️  Started job: {}", job_name);

        Ok(job_id)
    }

        /// Record job completion  
    pub async fn complete_job(  
        &self,  
        job_id: Uuid,  
        job_name: &str,  
    ) -> Result<(), sqlx::Error> {  
        sqlx::query(  
            r#"UPDATE job_executions   
               SET status = $1, completed_at = NOW(), duration_seconds = EXTRACT(EPOCH FROM (NOW() - started_at))::INT  
               WHERE id = $2"#,  
        )  
        .bind("completed")  
        .bind(job_id)  
        .execute(self.pool.as_ref())  
        .await?;  

        println!("✅ Completed job: {}", job_name);  

        Ok(())  
    }  

    /// Record job failure  
    pub async fn fail_job(  
        &self,  
        job_id: Uuid,  
        job_name: &str,  
        error: &str,  
    ) -> Result<(), sqlx::Error> {  
        sqlx::query(  
            r#"UPDATE job_executions   
               SET status = $1, completed_at = NOW(), error_message = $2, duration_seconds = EXTRACT(EPOCH FROM (NOW() - started_at))::INT  
               WHERE id = $3"#,  
        )  
        .bind("failed")  
        .bind(error)  
        .bind(job_id)  
        .execute(self.pool.as_ref())  
        .await?;  

        eprintln!("❌ Failed job: {}: {}", job_name, error);  

        Ok(())  
    }  

    /// Get job execution history  
    pub async fn get_job_history(  
        &self,  
        job_name: &str,  
        limit: i64,  
    ) -> Result<Vec<JobExecution>, sqlx::Error> {  
        sqlx::query_as::<_, (Uuid, String, String, DateTime<Utc>, Option<DateTime<Utc>>, Option<String>, Option<i32>)>(  
            r#"SELECT id, job_name, status, started_at, completed_at, error_message, duration_seconds  
               FROM job_executions  
               WHERE job_name = $1  
               ORDER BY started_at DESC  
               LIMIT $2"#,  
        )  
        .bind(job_name)  
        .bind(limit)  
        .fetch_all(self.pool.as_ref())  
        .await  
        .map(|rows| {  
            rows.into_iter()  
                .map(|(id, job_name, status_str, started_at, completed_at, error_message, duration_seconds)| {  
                    let status = match status_str.as_str() {  
                        "completed" => JobStatus::Completed,  
                        "failed" => JobStatus::Failed,  
                        "running" => JobStatus::Running,  
                        _ => JobStatus::Pending,  
                    };  

                    JobExecution {  
                        id,  
                        job_name,  
                        status,  
                        started_at,  
                        completed_at,  
                        error_message,  
                        duration_seconds,  
                    }  
                })  
                .collect()  
        })  
    }  
}
