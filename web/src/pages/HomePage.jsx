import React, { useEffect, useState } from "react";
import { useAuthContext } from "../context";
import { useMovie } from "../hooks";
import SearchBar from "../components/SearchBar";
import MovieGrid from "../components/features/MovieGrid";
import VideoPlayerModal from "../components/features/VideoPlayerModal";
import "../styles/pages/HomePage.css";

function HomePage() {
  const { logout } = useAuthContext();
  const {
    movies,
    fetchMovies,
    search,
    searchResults,
    favorites,
    toggleFavorite,
    isFavorited,
    saveProgress,
  } = useMovie();

  const [activeTab, setActiveTab] = useState("browse");
  const [selectedMovie, setSelectedMovie] = useState(null);
  const [selectedUrl, setSelectedUrl] = useState(null);

  useEffect(() => {
    if (activeTab === "browse") {
      fetchMovies();
    }
  }, [activeTab, fetchMovies]);

  const handleMovieClick = (movie) => {
    setSelectedMovie(movie);
    setSelectedUrl(movie.stream_url);
  };

  const getDisplayMovies = () => {
    switch (activeTab) {
      case "browse":
        return searchResults.length > 0 ? searchResults : movies;
      case "favorites":
        return favorites;
      case "watching":
        // This would come from watch history context
        return [];
      default:
        return movies;
    }
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>🎬 Moviey</h1>
        <button onClick={logout} className="logout-btn">
          Logout
        </button>
      </header>

      {selectedUrl && (
        <VideoPlayerModal
          movie={selectedMovie}
          src={selectedUrl}
          onClose={() => {
            setSelectedUrl(null);
            setSelectedMovie(null);
          }}
          onProgress={saveProgress}
        />
      )}

      <main className="app-main">
        {activeTab === "browse" && (
          <>
            <SearchBar onSearch={search.search} params={search.params} />
            {getDisplayMovies().length > 0 ? (
              <MovieGrid
                movies={getDisplayMovies()}
                onMovieClick={handleMovieClick}
                onFavoriteToggle={toggleFavorite}
                favorites={favorites}
              />
            ) : (
              <div className="empty-state">
                <p>No movies found. Try a different search.</p>
              </div>
            )}
          </>
        )}

        {activeTab === "watching" && (
          <>
            <h2 className="section-title">📺 Continue Watching</h2>
            <div className="empty-state">
              <p>Coming soon...</p>
            </div>
          </>
        )}

        {activeTab === "favorites" && (
          <>
            <h2 className="section-title">❤️ My Favorites</h2>
            {favorites.length > 0 ? (
              <MovieGrid
                movies={favorites}
                onMovieClick={handleMovieClick}
                onFavoriteToggle={toggleFavorite}
                favorites={favorites}
              />
            ) : (
              <div className="empty-state">
                <p>No favorites yet. Add movies to your watchlist!</p>
              </div>
            )}
          </>
        )}
      </main>

      <div className="tab-navigation">
        <button
          className={`tab-btn ${activeTab === "browse" ? "active" : ""}`}
          onClick={() => setActiveTab("browse")}
        >
          🏠 Browse
        </button>
        <button
          className={`tab-btn ${activeTab === "watching" ? "active" : ""}`}
          onClick={() => setActiveTab("watching")}
        >
          📺 Watching
        </button>
        <button
          className={`tab-btn ${activeTab === "favorites" ? "active" : ""}`}
          onClick={() => setActiveTab("favorites")}
        >
          ❤️ Favorites
        </button>
      </div>
    </div>
  );
}

export default HomePage;
