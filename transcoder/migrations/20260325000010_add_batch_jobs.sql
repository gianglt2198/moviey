-- Job Execution Tracking
CREATE TABLE IF NOT EXISTS job_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_name VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL, -- pending, running, completed, failed
    started_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    error_message TEXT,
    duration_seconds INT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- User Embeddings (for collaborative filtering)
CREATE TABLE IF NOT EXISTS user_embeddings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    profile_id UUID REFERENCES profiles(id) ON DELETE CASCADE UNIQUE,
    embedding_vector FLOAT8[] NOT NULL, -- Vector of movie preferences
    genre_weights JSONB, -- Genre preference distribution
    director_weights JSONB, -- Director preference distribution
    calculated_at TIMESTAMPTZ DEFAULT NOW(),
    version INT DEFAULT 1
);

-- Movie Embeddings (for content-based)
CREATE TABLE IF NOT EXISTS movie_embeddings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    movie_id UUID REFERENCES movies(id) ON DELETE CASCADE UNIQUE,
    feature_vector FLOAT8[] NOT NULL,
    genre_vector FLOAT8[],
    temporal_features JSONB,
    calculated_at TIMESTAMPTZ DEFAULT NOW(),
    version INT DEFAULT 1
);

-- Batch Job Errors
CREATE TABLE IF NOT EXISTS job_errors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_execution_id UUID REFERENCES job_executions(id) ON DELETE CASCADE,
    error_type VARCHAR(100),
    error_message TEXT,
    stack_trace TEXT,
    affected_records INT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Job Statistics
CREATE TABLE IF NOT EXISTS job_statistics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_name VARCHAR(100) NOT NULL UNIQUE,
    total_runs INT DEFAULT 0,
    successful_runs INT DEFAULT 0,
    failed_runs INT DEFAULT 0,
    avg_duration_seconds INT,
    last_run_at TIMESTAMPTZ,
    last_success_at TIMESTAMPTZ,
    last_failure_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_job_executions_name_started   
    ON job_executions(job_name, started_at DESC);  

CREATE INDEX IF NOT EXISTS idx_job_executions_status   
    ON job_executions(status);  
CREATE INDEX idx_user_embeddings_profile ON user_embeddings(profile_id);
CREATE INDEX idx_movie_embeddings_movie ON movie_embeddings(movie_id);
CREATE INDEX idx_job_errors_job_exec ON job_errors(job_execution_id);
CREATE INDEX idx_job_stats_name ON job_statistics(job_name);
