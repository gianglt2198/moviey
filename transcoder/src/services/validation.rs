pub struct RuleValidator;

impl RuleValidator {
    // Rule 1.1: Duration cannot exceed movie length
    // pub fn validate_duration(watch_duration: i32, total_duration: i32) -> Result<(), String> {
    //     if watch_duration > total_duration {
    //         return Err(format!(
    //             "Watch duration ({}) exceeds movie length ({})",
    //             watch_duration, total_duration
    //         ));
    //     }
    //     Ok(())
    // }

    // Rule 1.2: Minimum watch time
    // pub fn validate_minimum_watch(watch_duration: i32) -> Result<String, String> {
    //     if watch_duration < 10 {
    //         Ok("sampled".to_string()) // Mark as sampled, not rejected
    //     } else {
    //         Ok("valid".to_string())
    //     }
    // }

    // Rule 2.1: Timestamp cannot be in future
    // pub fn validate_timestamp(timestamp: i64) -> Result<(), String> {
    //     let now = Utc::now().timestamp();
    //     if timestamp > now {
    //         return Err("Timestamp cannot be in the future".to_string());
    //     }
    //     Ok(())
    // }

    // Rule 3.1: Playback speed validation
    pub fn validate_playback_speed(speed: f64) -> Result<f64, String> {
        if speed < 0.5 || speed > 3.0 {
            return Ok(1.0); // Clamp to normal speed  
        }
        Ok(speed)
    }

    // Rule 3.2: Interrupted count must be non-negative
    pub fn validate_interrupt_count(count: i32) -> Result<(), String> {
        if count < 0 {
            return Err("Interrupted count cannot be negative".to_string());
        }
        Ok(())
    }

    // Rule 3.3: Consistency check
    // pub fn validate_completion_consistency(
    //     completed: bool,
    //     completion_percentage: f64,
    // ) -> Result<(), String> {
    //     if completed && completion_percentage < 95.0 {
    //         return Err(format!(
    //             "Movie marked completed but completion rate is only {:.1}%",
    //             completion_percentage
    //         ));
    //     }
    //     Ok(())
    // }

    // Rule 3.4: Device type validation
    pub fn validate_device_type(device: &str) -> Result<(), String> {
        let allowed = vec!["web", "mobile", "tablet", "smart_tv"];
        if !allowed.contains(&device) {
            return Err(format!("Invalid device type: {}", device));
        }
        Ok(())
    }

    // Calculate completion percentage
    pub fn calculate_completion_percentage(watch_duration: i32, total_duration: i32) -> f64 {
        if total_duration == 0 {
            return 0.0;
        }
        ((watch_duration as f64 / total_duration as f64) * 100.0)
            .min(100.0)
            .max(0.0)
    }

    // Determine completion reason
    pub fn determine_completion_reason(completed: bool, completion_percentage: f64) -> String {
        if completed || completion_percentage >= 95.0 {
            "finished".to_string()
        } else if completion_percentage >= 20.0 {
            "abandoned".to_string()
        } else {
            "sampled".to_string()
        }
    }

    // Flag suspicious records
    // pub async fn create_quality_flag(
    //     pool: &PgPool,
    //     watch_history_id: Uuid,
    //     flag_type: &str,
    //     flag_severity: &str,
    //     description: &str,
    // ) -> Result<(), sqlx::Error> {
    //     sqlx::query(
    //         "INSERT INTO data_quality_flags
    //          (watch_history_id, flag_type, flag_severity, description)
    //          VALUES ($1, $2, $3, $4)",
    //     )
    //     .bind(watch_history_id)
    //     .bind(flag_type)
    //     .bind(flag_severity)
    //     .bind(description)
    //     .execute(pool)
    //     .await?;

    //     Ok(())
    // }
}
