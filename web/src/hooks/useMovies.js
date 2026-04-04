import { useState, useCallback } from "react";
import { movieService } from "../services/api";

export const useMovies = () => {
  const [movies, setMovies] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [currentMovie, setCurrentMovie] = useState(null);

  /**
   * Fetch all movies
   */
  const fetchMovies = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const { data } = await movieService.getMovies();
      setMovies(data);
      return data;
    } catch (err) {
      setError(err);
      console.error("Failed to fetch movies:", err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Search movies with filters
   */
  const searchMovies = useCallback(async (params) => {
    setLoading(true);
    setError(null);

    try {
      const { data } = await movieService.searchMovies(params);
      setMovies(data);
      return data;
    } catch (err) {
      setError(err);
      console.error("Search failed:", err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Get detailed information about a movie
   */
  const getMovieDetail = useCallback(async (movieId) => {
    setLoading(true);
    setError(null);

    try {
      const { data } = await movieService.getMovieDetail(movieId);
      setCurrentMovie(data);
      return data;
    } catch (err) {
      setError(err);
      console.error("Failed to fetch movie detail:", err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Find movie in current list by ID
   */
  const findMovie = useCallback(
    (movieId) => {
      return movies.find((movie) => movie.id === movieId);
    },
    [movies],
  );

  /**
   * Clear current movie
   */
  const clearCurrentMovie = useCallback(() => {
    setCurrentMovie(null);
  }, []);

  return {
    movies,
    currentMovie,
    loading,
    error,
    fetchMovies,
    searchMovies,
    getMovieDetail,
    findMovie,
    clearCurrentMovie,
  };
};
