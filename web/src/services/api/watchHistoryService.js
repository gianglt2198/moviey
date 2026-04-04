import apiClient from "./apiClient";
import { API_ENDPOINTS } from "../../constants/api";

const watchHistoryService = {
  /**
   * Get user's watch history
   * @returns {Promise<Array>} Array of watched movies
   */
  getWatchHistory: () => {
    return apiClient.get(API_ENDPOINTS.WATCH_HISTORY.LIST);
  },

  /**
   * Save watch progress for a movie
   * @param {Object} data - Progress data
   * @param {number|string} data.movie_id - Movie ID
   * @param {number} data.position_seconds - Current position
   * @param {boolean} data.completed - Whether movie is completed
   * @returns {Promise<Object>} Save response
   */
  saveProgress: (data) => {
    return apiClient.post(API_ENDPOINTS.WATCH_HISTORY.SAVE, data);
  },

  /**
   * Save enhanced watch history with metadata
   * @param {Object} data - Enhanced history data
   * @returns {Promise<Object>} Save response
   */
  saveEnhancedHistory: (data) => {
    return apiClient.post(API_ENDPOINTS.WATCH_HISTORY.SAVE_ENHANCED, data);
  },
};

export default watchHistoryService;
