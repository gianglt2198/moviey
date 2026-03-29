CREATE TABLE IF NOT EXISTS user_segments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    profile_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
    segment_type VARCHAR NOT NULL,
    segment_score DECIMAL(5,2) DEFAULT 0,
    last_calculated_at TIMESTAMPTZ DEFAULT NOW(),
    valid_until TIMESTAMPTZ NOT NULL,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(profile_id)
);

CREATE INDEX IF NOT EXISTS idx_user_segments_profile 
  ON user_segments(profile_id);
CREATE INDEX IF NOT EXISTS idx_user_segments_type 
  ON user_segments(segment_type);
