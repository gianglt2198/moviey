import { useMoviesContext } from "../context/MoviesContext";
import { useFavoritesContext } from "../context/FavoritesContext";
import { useWatchHistoryContext } from "../context/WatchHistoryContext";

/**
 * Combined hook for movie-related operations
 * Provides movies, favorites, and watch history together
 */
export const useMovie = () => {
  const moviesContext = useMoviesContext();
  const favoritesContext = useFavoritesContext();
  const historyContext = useWatchHistoryContext();

  return {
    // Movies
    movies: moviesContext.movies.movies,
    currentMovie: moviesContext.movies.currentMovie,
    fetchMovies: moviesContext.movies.fetchMovies,
    getMovieDetail: moviesContext.movies.getMovieDetail,
    moviesLoading: moviesContext.movies.loading,
    moviesError: moviesContext.movies.error,

    // Search
    searchResults: moviesContext.search.results,
    search: moviesContext.search.search,
    searchParams: moviesContext.search.params,
    clearSearch: moviesContext.search.clearSearch,
    searchLoading: moviesContext.search.loading,
    searchError: moviesContext.search.error,

    // Favorites
    favorites: favoritesContext.favorites,
    toggleFavorite: favoritesContext.toggleFavorite,
    isFavorited: favoritesContext.isFavorited,
    favoritesLoading: favoritesContext.loading,

    // Watch History
    watchHistory: historyContext.watchHistory,
    saveProgress: historyContext.saveProgress,
    saveEnhancedHistory: historyContext.saveEnhancedHistory,
    historyLoading: historyContext.loading,
  };
};
