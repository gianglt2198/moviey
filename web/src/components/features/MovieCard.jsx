import React from "react";
import "../../styles/components/MovieCard.css";

function MovieCard({ movie, isFavorited, onMovieClick, onFavoriteClick }) {
  return (
    <div className="movie-card">
      <div className="thumbnail-container">
        <img
          src={movie.thumbnail_url}
          alt={movie.title}
          className="movie-thumb"
          onClick={onMovieClick}
        />
        {movie.duration && (
          <span className="duration-tag">{movie.duration}</span>
        )}
        {movie.rating && <span className="rating-tag">⭐ {movie.rating}</span>}
        <button
          className={`favorite-btn ${isFavorited ? "favorited" : ""}`}
          onClick={(e) => {
            e.stopPropagation();
            onFavoriteClick();
          }}
          title={isFavorited ? "Remove from favorites" : "Add to favorites"}
        >
          {isFavorited ? "❤️" : "🤍"}
        </button>
      </div>
      <h3>{movie.title}</h3>
      {movie.genre && <p className="genre">{movie.genre}</p>}
      {movie.director && <p className="director">📺 {movie.director}</p>}
      <button onClick={onMovieClick} className="watch-btn">
        Watch Now
      </button>
    </div>
  );
}

export default MovieCard;
