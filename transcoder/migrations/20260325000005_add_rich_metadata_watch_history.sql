-- Add new columns to watch_history for richer behavioral signals  
ALTER TABLE watch_history  
ADD COLUMN IF NOT EXISTS watch_duration_seconds INTEGER DEFAULT 0,  
ADD COLUMN IF NOT EXISTS total_movie_duration_seconds INTEGER DEFAULT 0,  
ADD COLUMN IF NOT EXISTS completion_percentage DECIMAL(5,2) DEFAULT 0,  
ADD COLUMN IF NOT EXISTS watch_quality VARCHAR DEFAULT '1080p',  
ADD COLUMN IF NOT EXISTS interrupted_count INTEGER DEFAULT 0,  
ADD COLUMN IF NOT EXISTS last_session_resumed_at TIMESTAMPTZ NULL,  
ADD COLUMN IF NOT EXISTS playback_speed DECIMAL(2,1) DEFAULT 1.0,  
ADD COLUMN IF NOT EXISTS device_type VARCHAR DEFAULT 'web',  
ADD COLUMN IF NOT EXISTS completion_reason VARCHAR DEFAULT 'abandoned',  
ADD COLUMN IF NOT EXISTS flagged_for_review BOOLEAN DEFAULT FALSE;  

-- Create indexes for frequently queried columns  
CREATE INDEX IF NOT EXISTS idx_watch_history_completion_percentage   
  ON watch_history(completion_percentage);  
CREATE INDEX IF NOT EXISTS idx_watch_history_flagged   
  ON watch_history(flagged_for_review);  
CREATE INDEX IF NOT EXISTS idx_watch_history_device_type   
  ON watch_history(device_type);  