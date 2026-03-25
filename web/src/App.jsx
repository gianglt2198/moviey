import React, { useEffect, useState } from "react";
import axios from "axios";
import VideoPlayer from "./pages/VideoPlayer";
import "./App.css";

// Setup Axios interceptor for JWT
axios.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Token expired or invalid
      localStorage.removeItem("token");
      window.location.href = "/login";
    }
    return Promise.reject(error);
  },
);

function App() {
  const [movies, setMovies] = useState([]);
  const [selectedUrl, setSelectedUrl] = useState(null);
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  // Set auth token on app load
  useEffect(() => {
    const token = localStorage.getItem("token");
    if (token) {
      axios.defaults.headers.common["Authorization"] = `Bearer ${token}`;
      setIsLoggedIn(true);
      fetchMovies();
    }
  }, []);

  const fetchMovies = async () => {
    try {
      const res = await axios.get("http://localhost:3000/api/movies");
      setMovies(res.data);
    } catch (err) {
      console.error("Failed to fetch movies:", err);
    }
  };

  const handleLogin = async () => {
    try {
      const res = await axios.post("http://localhost:3000/api/auth/login", {
        email,
        password,
      });
      localStorage.setItem("token", res.data.token);
      axios.defaults.headers.common["Authorization"] =
        `Bearer ${res.data.token}`;
      setIsLoggedIn(true);
      setEmail("");
      setPassword("");
      fetchMovies();
    } catch (err) {
      alert(
        "Login failed: " + err.response?.data?.message || "Invalid credentials",
      );
    }
  };

  const handleLogout = () => {
    localStorage.removeItem("token");
    delete axios.defaults.headers.common["Authorization"];
    setIsLoggedIn(false);
    setMovies([]);
  };

  if (!isLoggedIn) {
    return (
      <div className="login-container">
        <h1>Login to Moviey</h1>
        <input
          type="email"
          placeholder="Email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
        <input
          type="password"
          placeholder="Password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        <button onClick={handleLogin}>Login</button>
      </div>
    );
  }

  return (
    <div className="app">
      <header>
        <h1>Moviey Streaming</h1>
        <button onClick={handleLogout}>Logout</button>
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
