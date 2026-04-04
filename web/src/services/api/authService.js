import apiClient from "./apiClient";
import { API_ENDPOINTS } from "../../constants/api";

const authService = {
  /**
   * Login user with email and password
   * @param {string} email - User email
   * @param {string} password - User password
   * @returns {Promise<Object>} Auth response with token
   */
  login: (email, password) => {
    return apiClient.post(API_ENDPOINTS.AUTH.LOGIN, { email, password });
  },

  /**
   * Register a new user
   * @param {Object} userData - User registration data
   * @returns {Promise<Object>} Registration response
   */
  register: (userData) => {
    return apiClient.post(API_ENDPOINTS.AUTH.REGISTER, userData);
  },

  /**
   * Logout user
   */
  logout: () => {
    localStorage.removeItem("token");
    delete apiClient.defaults.headers.common["Authorization"];
  },

  /**
   * Get token from localStorage
   * @returns {string|null} JWT token or null
   */
  getToken: () => {
    return localStorage.getItem("token");
  },

  /**
   * Check if user is authenticated
   * @returns {boolean} True if authenticated
   */
  isAuthenticated: () => {
    return !!localStorage.getItem("token");
  },
};

export default authService;
