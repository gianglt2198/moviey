CREATE TABLE IF NOT EXISTS user_behavior_snapshot (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    profile_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
    snapshot_date DATE NOT NULL,
    movies_watched_period INTEGER DEFAULT 0,
    total_watch_time_minutes INTEGER DEFAULT 0,
    avg_completion_rate DECIMAL(5,2) DEFAULT 0,
    preferred_genre VARCHAR,
    preferred_time_of_day VARCHAR,
    device_most_used VARCHAR,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(profile_id, snapshot_date)
);

CREATE INDEX IF NOT EXISTS idx_behavior_snapshot_profile_date 
  ON user_behavior_snapshot(profile_id, snapshot_date DESC);
CREATE INDEX IF NOT EXISTS idx_behavior_snapshot_date 
  ON user_behavior_snapshot(snapshot_date DESC);
