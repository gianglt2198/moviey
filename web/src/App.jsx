import React, { useEffect, useState } from 'react';
import axios from 'axios';
import VideoPlayer from './pages/VideoPlayer';
import SearchBar from './components/SearchBar';
import './App.css';

axios.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  },
);

function App() {
  const [movies, setMovies] = useState([]);
  const [watchHistory, setWatchHistory] = useState([]);
  const [favorites, setFavorites] = useState([]);
  const [selectedMovie, setSelectedMovie] = useState(null);
  const [selectedUrl, setSelectedUrl] = useState(null);
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [activeTab, setActiveTab] = useState('browse'); // browse, watching, favorites

  useEffect(() => {
    const token = localStorage.getItem('token');
    if (token) {
      axios.defaults.headers.common['Authorization'] = `Bearer ${token}`;
      setIsLoggedIn(true);
      fetchMovies();
      fetchWatchHistory();
      fetchFavorites();
    }
  }, []);

  const fetchMovies = async () => {
    try {
      const res = await axios.get('http://localhost:3000/api/movies');
      setMovies(res.data);
    } catch (err) {
      console.error('Failed to fetch movies:', err);
    }
  };

  const fetchWatchHistory = async () => {
    try {
      const res = await axios.get('http://localhost:3000/api/watch-history');
      console.log('Fetched watch history:', res.data);
      setWatchHistory(res.data);
    } catch (err) {
      console.error('Failed to fetch watch history:', err);
    }
  };

  const fetchFavorites = async () => {
    try {
      const res = await axios.get('http://localhost:3000/api/favorites');
      setFavorites(res.data);
    } catch (err) {
      console.error('Failed to fetch favorites:', err);
    }
  };

  const handleLogin = async () => {
    try {
      const res = await axios.post('http://localhost:3000/api/auth/login', {
        email,
        password,
      });
      localStorage.setItem('token', res.data.token);
      axios.defaults.headers.common['Authorization'] =
        `Bearer ${res.data.token}`;
      setIsLoggedIn(true);
      setEmail('');
      setPassword('');
      fetchMovies();
      fetchWatchHistory();
      fetchFavorites();
    } catch (err) {
      alert('Login failed: Invalid credentials');
    }
  };

  const handleLogout = () => {
    localStorage.removeItem('token');
    delete axios.defaults.headers.common['Authorization'];
    setIsLoggedIn(false);
    setMovies([]);
    setWatchHistory([]);
    setFavorites([]);
  };

  const handleMovieClick = async (movie) => {
    setSelectedMovie(movie);
    setSelectedUrl(movie.stream_url);
  };

  const handleToggleFavorite = async (movie) => {
    try {
      await axios.post('http://localhost:3000/api/favorites/toggle', {
        movie_id: movie.id,
      });
      await fetchFavorites();
    } catch (err) {
      console.error('Failed to toggle favorite:', err);
    }
  };

  const handleSaveProgress = async (positionSeconds) => {
    if (!selectedMovie) return;
    try {
      await axios.post('http://localhost:3000/api/watch-progress', {
        movie_id: selectedMovie.id,
        position_seconds: positionSeconds,
        completed: false,
      });
    } catch (err) {
      console.error('Failed to save progress:', err);
    }
  };

  const isFavorited = (movieId) => {
    return favorites.some((fav) => fav.id === movieId);
  };

  const renderMovieGrid = (movieList, showMetadata = false) => (
    <div className="movie-grid">
      {movieList.map((movie) => (
        <div key={movie.id} className="movie-card">
          <div className="thumbnail-container">
            <img
              src={movie.thumbnail_url}
              alt={movie.title}
              className="movie-thumb"
              onClick={() => handleMovieClick(movie)}
            />
            <span className="duration-tag">{movie.duration}</span>
            {movie.rating && (
              <span className="rating-tag">⭐ {movie.rating}</span>
            )}
            <button
              className={`favorite-btn ${
                isFavorited(movie.id) ? 'favorited' : ''
              }`}
              onClick={(e) => {
                e.stopPropagation();
                handleToggleFavorite(movie);
              }}
              title="Add to favorites"
            >
              ❤️
            </button>
          </div>
          <h3>{movie.title}</h3>
          {showMetadata && movie.genre && (
            <p className="genre">{movie.genre}</p>
          )}
          {showMetadata && movie.director && (
            <p className="director">📺 {movie.director}</p>
          )}
          <button onClick={() => handleMovieClick(movie)} className="watch-btn">
            Watch Now
          </button>
        </div>
      ))}
    </div>
  );

  if (!isLoggedIn) {
    return (
      <div className="login-container">
        <div className="login-box">
          <h1>🎬 Moviey</h1>
          <p className="tagline">Your Personal Streaming Hub</p>
          <input
            type="email"
            placeholder="Email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            onKeyPress={(e) => e.key === 'Enter' && handleLogin()}
          />
          <input
            type="password"
            placeholder="Password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            onKeyPress={(e) => e.key === 'Enter' && handleLogin()}
          />
          <button onClick={handleLogin}>Login</button>
          <p className="signup-hint">Demo: test@example.com / TestPass123</p>
        </div>
      </div>
    );
  }

  return (
    <div className="app">
      <header className="app-header">
        <h1>🎬 Moviey</h1>
        <button onClick={handleLogout} className="logout-btn">
          Logout
        </button>
      </header>

      {selectedUrl && (
        <div className="player-modal">
          <div className="player-wrapper">
            <button
              className="close-btn"
              onClick={() => {
                setSelectedUrl(null);
                setSelectedMovie(null);
              }}
            >
              ✕
            </button>
            <VideoPlayer src={selectedUrl} onProgress={handleSaveProgress} />
            {selectedMovie && (
              <div className="movie-details">
                <h2>{selectedMovie.title}</h2>
                <p>{selectedMovie.description}</p>
                {selectedMovie.genre && (
                  <span className="badge">{selectedMovie.genre}</span>
                )}
              </div>
            )}
          </div>
        </div>
      )}

      <main className="app-main">
        {activeTab === 'browse' && (
          <>
            <SearchBar onSearch={setMovies} />
            {renderMovieGrid(movies, true)}
          </>
        )}

        {activeTab === 'watching' && (
          <>
            <h2 className="section-title">📺 Continue Watching</h2>
            {watchHistory.length > 0 ? (
              renderMovieGrid(watchHistory, true)
            ) : (
              <div className="empty-state">
                <p>No watch history yet. Start watching a movie!</p>
              </div>
            )}
          </>
        )}

        {activeTab === 'favorites' && (
          <>
            <h2 className="section-title">❤️ My Favorites</h2>
            {favorites.length > 0 ? (
              renderMovieGrid(favorites, true)
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
          className={`tab-btn ${activeTab === 'browse' ? 'active' : ''}`}
          onClick={() => setActiveTab('browse')}
        >
          🏠 Browse
        </button>
        <button
          className={`tab-btn ${activeTab === 'watching' ? 'active' : ''}`}
          onClick={() => setActiveTab('watching')}
        >
          📺 Continue Watching
        </button>
        <button
          className={`tab-btn ${activeTab === 'favorites' ? 'active' : ''}`}
          onClick={() => setActiveTab('favorites')}
        >
          ❤️ Favorites
        </button>
      </div>
    </div>
  );
}

export default App;
