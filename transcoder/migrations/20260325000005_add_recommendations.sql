-- Create user ratings table
CREATE TABLE IF NOT EXISTS user_ratings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    profile_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
    movie_id UUID REFERENCES movies(id) ON DELETE CASCADE,
    rating DECIMAL(2,1) NOT NULL CHECK (rating >= 0.5 AND rating <= 5.0),
    rated_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(profile_id, movie_id),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Content-based similarity cache table
CREATE TABLE IF NOT EXISTS movie_similarity (
    movie_id_1 UUID REFERENCES movies(id) ON DELETE CASCADE,
    movie_id_2 UUID REFERENCES movies(id) ON DELETE CASCADE,
    similarity_score DECIMAL(3,2) NOT NULL CHECK (similarity_score >= 0.0 AND similarity_score <= 1.0),
    calculated_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (movie_id_1, movie_id_2)
);

-- User similarity matrix table
CREATE TABLE IF NOT EXISTS user_similarity (
    user_id_1 UUID REFERENCES users(id) ON DELETE CASCADE,
    user_id_2 UUID REFERENCES users(id) ON DELETE CASCADE,
    similarity_score DECIMAL(3,2) NOT NULL CHECK (similarity_score >= 0.0 AND similarity_score <= 1.0),
    calculated_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (user_id_1, user_id_2)
);

-- Cached recommendations (updated nightly)
CREATE TABLE IF NOT EXISTS recommendations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    profile_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
    recommended_movies UUID[] NOT NULL,
    scores DECIMAL(3,2)[] NOT NULL,
    algorithm VARCHAR(50) NOT NULL, -- 'content_based', 'collaborative', 'hybrid'
    calculated_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    UNIQUE(profile_id, algorithm)
);

-- Recommendation feedback tracking
CREATE TABLE IF NOT EXISTS recommendation_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    profile_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
    movie_id UUID REFERENCES movies(id) ON DELETE CASCADE,
    recommendation_source VARCHAR(50),
    clicked BOOLEAN DEFAULT FALSE,
    watched BOOLEAN DEFAULT FALSE,
    rating DECIMAL(2,1),
    feedback_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_user_ratings_profile ON user_ratings(profile_id);
CREATE INDEX IF NOT EXISTS idx_user_ratings_movie ON user_ratings(movie_id);
CREATE INDEX IF NOT EXISTS idx_movie_similarity_1 ON movie_similarity(movie_id_1);
CREATE INDEX IF NOT EXISTS idx_user_similarity_1 ON user_similarity(user_id_1);
CREATE INDEX IF NOT EXISTS idx_recommendations_profile ON recommendations(profile_id);
CREATE INDEX IF NOT EXISTS idx_recommendations_expires ON recommendations(expires_at);
