import React from "react";
import MovieCard from "./MovieCard";
import "../../styles/components/MovieGrid.css";

function MovieGrid({ movies, onMovieClick, onFavoriteToggle, favorites = [] }) {
  if (!movies || movies.length === 0) {
    return <div className="empty-state">No movies found</div>;
  }

  return (
    <div className="movie-grid">
      {movies.map((movie) => (
        <MovieCard
          key={movie.id}
          movie={movie}
          isFavorited={favorites.some((fav) => fav.id === movie.id)}
          onMovieClick={() => onMovieClick(movie)}
          onFavoriteClick={() => onFavoriteToggle(movie.id)}
        />
      ))}
    </div>
  );
}

export default MovieGrid;
