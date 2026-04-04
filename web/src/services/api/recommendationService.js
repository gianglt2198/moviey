import apiClient from "./apiClient";
import { API_ENDPOINTS } from "../../constants/api";

const recommendationService = {
  /**
   * Generate recommendations for user
   * @param {number|string} userId - User ID
   * @returns {Promise<Object>} Generated recommendations
   */
  generateRecommendations: (userId) => {
    return apiClient.post(API_ENDPOINTS.RECOMMENDATIONS.GENERATE(userId));
  },

  /**
   * Get cached recommendations for user
   * @param {number|string} userId - User ID
   * @returns {Promise<Array>} User recommendations
   */
  getRecommendations: (userId) => {
    return apiClient.get(API_ENDPOINTS.RECOMMENDATIONS.GET(userId));
  },

  /**
   * Get similar movies for a specific movie
   * @param {number|string} movieId - Movie ID
   * @returns {Promise<Array>} Similar movies
   */
  getSimilarMovies: (movieId) => {
    return apiClient.get(API_ENDPOINTS.RECOMMENDATIONS.SIMILAR(movieId));
  },

  /**
   * Log recommendation feedback
   * @param {Object} data - Feedback data
   * @param {number|string} data.movie_id - Movie ID
   * @param {string} data.action - User action (click, watch, skip, etc.)
   * @param {number} data.watch_duration_seconds - Watch duration
   * @returns {Promise<Object>} Feedback response
   */
  sendFeedback: (data) => {
    return apiClient.post(API_ENDPOINTS.RECOMMENDATIONS.FEEDBACK, data);
  },
};

export default recommendationService;
