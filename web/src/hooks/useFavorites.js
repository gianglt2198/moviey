import { useState, useCallback } from "react";
import { favoriteService } from "../services/api";

export const useFavorites = () => {
  const [favorites, setFavorites] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  /**
   * Fetch user's favorite movies
   */
  const fetchFavorites = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const { data } = await favoriteService.getFavorites();
      setFavorites(data);
      return data;
    } catch (err) {
      setError(err);
      console.error("Failed to fetch favorites:", err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Toggle favorite status for a movie
   */
  const toggleFavorite = useCallback(
    async (movieId) => {
      try {
        await favoriteService.toggleFavorite(movieId);

        // Update local state optimistically
        setFavorites((prev) => {
          const isFavorited = prev.some((fav) => fav.id === movieId);
          if (isFavorited) {
            return prev.filter((fav) => fav.id !== movieId);
          }
          return prev;
        });

        // Refresh from server
        await fetchFavorites();
      } catch (err) {
        setError(err);
        console.error("Failed to toggle favorite:", err);
        throw err;
      }
    },
    [fetchFavorites],
  );

  /**
   * Check if movie is in favorites
   */
  const isFavorited = useCallback(
    (movieId) => {
      return favorites.some((fav) => fav.id === movieId);
    },
    [favorites],
  );

  /**
   * Clear all favorites from local state
   */
  const clearFavorites = useCallback(() => {
    setFavorites([]);
  }, []);

  return {
    favorites,
    loading,
    error,
    fetchFavorites,
    toggleFavorite,
    isFavorited,
    clearFavorites,
  };
};
