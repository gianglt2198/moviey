import { useState, useCallback } from "react";
import { watchHistoryService } from "../services/api";

export const useWatchHistory = () => {
  const [watchHistory, setWatchHistory] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  /**
   * Fetch user's watch history
   */
  const fetchWatchHistory = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const { data } = await watchHistoryService.getWatchHistory();
      setWatchHistory(data);
      return data;
    } catch (err) {
      setError(err);
      console.error("Failed to fetch watch history:", err);
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  /**
   * Save watch progress for a movie
   */
  const saveProgress = useCallback(
    async (movieId, positionSeconds, completed = false) => {
      try {
        const response = await watchHistoryService.saveProgress({
          movie_id: movieId,
          position_seconds: positionSeconds,
          completed,
        });

        // Refresh watch history
        await fetchWatchHistory();
        return response.data;
      } catch (err) {
        setError(err);
        console.error("Failed to save progress:", err);
        throw err;
      }
    },
    [fetchWatchHistory],
  );

  /**
   * Save enhanced watch history with metadata
   */
  const saveEnhancedHistory = useCallback(
    async (movieId, metadata) => {
      try {
        const response = await watchHistoryService.saveEnhancedHistory({
          movie_id: movieId,
          ...metadata,
        });
        await fetchWatchHistory();
        return response.data;
      } catch (err) {
        setError(err);
        console.error("Failed to save enhanced history:", err);
        throw err;
      }
    },
    [fetchWatchHistory],
  );

  /**
   * Get movie from watch history by ID
   */
  const getHistoryItem = useCallback(
    (movieId) => {
      return watchHistory.find((item) => item.id === movieId);
    },
    [watchHistory],
  );

  /**
   * Clear watch history
   */
  const clearHistory = useCallback(() => {
    setWatchHistory([]);
  }, []);

  return {
    watchHistory,
    loading,
    error,
    fetchWatchHistory,
    saveProgress,
    saveEnhancedHistory,
    getHistoryItem,
    clearHistory,
  };
};
