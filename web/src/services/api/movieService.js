import apiClient from "./apiClient";
import { API_ENDPOINTS } from "../../constants/api";

const movieService = {
  /**
   * Get all movies
   * @returns {Promise<Array>} Array of movie objects
   */
  getMovies: () => {
    return apiClient.get(API_ENDPOINTS.MOVIES.LIST);
  },

  /**
   * Search movies with filters
   * @param {Object} params - Search parameters
   * @param {string} params.q - Search query
   * @param {string} params.genre - Genre filter
   * @param {string} params.sort - Sort option (recent, rating, title)
   * @returns {Promise<Array>} Filtered movie results
   */
  searchMovies: (params) => {
    return apiClient.get(API_ENDPOINTS.MOVIES.SEARCH, { params });
  },

  /**
   * Get detailed information about a specific movie
   * @param {number|string} movieId - Movie ID
   * @returns {Promise<Object>} Movie details
   */
  getMovieDetail: (movieId) => {
    return apiClient.get(API_ENDPOINTS.MOVIES.DETAIL(movieId));
  },

  /**
   * Upload a new movie
   * @param {FormData} formData - Movie data with file
   * @returns {Promise<Object>} Upload response
   */
  uploadMovie: (formData) => {
    return apiClient.post(API_ENDPOINTS.MOVIES.UPLOAD, formData, {
      headers: { "Content-Type": "multipart/form-data" },
    });
  },
};

export default movieService;
