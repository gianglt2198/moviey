import React, { useEffect, useState } from 'react';
import axios from 'axios';
import VideoPlayer from './pages/VideoPlayer';
import './App.css';

function App() {
  const [movies, setMovies] = useState([]);
  const [selectedUrl, setSelectedUrl] = useState(null);

  useEffect(() => {
    axios
      .get('http://localhost:3000/api/movies')
      .then((res) => setMovies(res.data))
      .catch((err) => console.error('API Error:', err));
  }, []);

  return (
    <div className="app">
      <header>
        <h1>Moviey Streaming</h1>
      </header>

      <main>
        {selectedUrl && (
          <div className="player-container">
            <VideoPlayer src={selectedUrl} />
            <button onClick={() => setSelectedUrl(null)}>Close Player</button>
          </div>
        )}

        <div className="movie-grid">
          {movies.map((movie) => (
            <div
              key={movie.id}
              className="movie-card"
              onClick={() => setSelectedUrl(movie.stream_url)}
            >
              <div className="thumbnail-container">
                <img
                  src={movie.thumbnail_url}
                  alt={movie.title}
                  className="movie-thumb"
                />
                <span className="duration-tag">{movie.duration}</span>
              </div>
              <h3>{movie.title}</h3>
              <button>Watch Now</button>
            </div>
          ))}
        </div>
      </main>
    </div>
  );
}

export default App;
