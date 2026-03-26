-- Favorites Table (Watchlist)
CREATE TABLE IF NOT EXISTS favorites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    profile_id UUID REFERENCES profiles(id) ON DELETE CASCADE,
    movie_id UUID REFERENCES movies(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(profile_id, movie_id)
);

-- Add metadata columns to movies if not exists
ALTER TABLE movies
ADD COLUMN IF NOT EXISTS genre TEXT DEFAULT 'Unknown',
ADD COLUMN IF NOT EXISTS director TEXT DEFAULT 'Unknown',
ADD COLUMN IF NOT EXISTS release_year INTEGER DEFAULT 2024,
ADD COLUMN IF NOT EXISTS rating DECIMAL(3,1) DEFAULT 0.0,
ADD COLUMN IF NOT EXISTS description TEXT DEFAULT '';

-- Create index for faster searches
CREATE INDEX IF NOT EXISTS idx_movies_title ON movies(title);
CREATE INDEX IF NOT EXISTS idx_movies_genre ON movies(genre);
CREATE INDEX IF NOT EXISTS idx_favorites_profile ON favorites(profile_id);
