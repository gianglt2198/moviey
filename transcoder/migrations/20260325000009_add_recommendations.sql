-- Recommendations Cache Table
CREATE TABLE IF NOT EXISTS recommendations_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    profile_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
    movie_id UUID REFERENCES movies(id) ON DELETE CASCADE,
    collab_score DECIMAL(5,3) NOT NULL,
    content_score DECIMAL(5,3) NOT NULL,
    hybrid_score DECIMAL(5,3) NOT NULL,
    reason TEXT,
    generated_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    UNIQUE(profile_id, movie_id)
);

-- Movie Similarity Scores
CREATE TABLE IF NOT EXISTS movie_similarity_scores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    movie_a_id UUID REFERENCES movies(id) ON DELETE CASCADE,
    movie_b_id UUID REFERENCES movies(id) ON DELETE CASCADE,
    similarity_score DECIMAL(5,3) NOT NULL,
    similarity_type VARCHAR(50), -- 'genre', 'director', 'content'
    calculated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(movie_a_id, movie_b_id, similarity_type)
);

-- User Preferences
CREATE TABLE IF NOT EXISTS user_preferences (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    profile_id UUID REFERENCES profiles(id) ON DELETE CASCADE UNIQUE,
    preferred_genres TEXT[],
    preferred_directors TEXT[],
    avg_completion_rate DECIMAL(5,2),
    watch_frequency INT DEFAULT 0,
    last_updated TIMESTAMPTZ DEFAULT NOW()
);

-- Recommendation Feedback
CREATE TABLE IF NOT EXISTS recommendation_feedback (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    profile_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
    movie_id UUID REFERENCES movies(id) ON DELETE CASCADE,
    action VARCHAR(50), -- 'click', 'watch', 'not_interested', 'dislike'
    watch_duration_seconds INT,
    feedback_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_recommendations_cache_profile_expires 
    ON recommendations_cache(profile_id, expires_at DESC);
CREATE INDEX IF NOT EXISTS idx_movie_similarity_movie_a 
    ON movie_similarity_scores(movie_a_id);
CREATE INDEX IF NOT EXISTS idx_movie_similarity_movie_b 
    ON movie_similarity_scores(movie_b_id);
CREATE INDEX IF NOT EXISTS idx_user_preferences_profile 
    ON user_preferences(profile_id);
CREATE INDEX IF NOT EXISTS idx_recommendation_feedback_profile_movie 
    ON recommendation_feedback(profile_id, movie_id);
