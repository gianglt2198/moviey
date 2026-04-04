import apiClient from "./apiClient";
import { API_ENDPOINTS } from "../../constants/api";

const favoriteService = {
  /**
   * Get user's favorite movies
   * @returns {Promise<Array>} Array of favorite movies
   */
  getFavorites: () => {
    return apiClient.get(API_ENDPOINTS.FAVORITES.LIST);
  },

  /**
   * Toggle favorite status for a movie
   * @param {number|string} movieId - Movie ID
   * @returns {Promise<Object>} Toggle response
   */
  toggleFavorite: (movieId) => {
    return apiClient.post(API_ENDPOINTS.FAVORITES.TOGGLE, {
      movie_id: movieId,
    });
  },
};

export default favoriteService;
