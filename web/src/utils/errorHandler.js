import { HTTP_STATUS } from "../constants/api";

/**
 * Format error message from API response
 * @param {Error} error - Axios error object
 * @returns {string} Formatted error message
 */
export const getErrorMessage = (error) => {
  if (error.response?.data?.message) {
    return error.response.data.message;
  }

  switch (error.response?.status) {
    case HTTP_STATUS.BAD_REQUEST:
      return "Invalid request. Please check your input.";
    case HTTP_STATUS.UNAUTHORIZED:
      return "Your session has expired. Please login again.";
    case HTTP_STATUS.FORBIDDEN:
      return "You do not have permission to perform this action.";
    case HTTP_STATUS.NOT_FOUND:
      return "The requested resource was not found.";
    case HTTP_STATUS.SERVER_ERROR:
      return "Server error occurred. Please try again later.";
    case HTTP_STATUS.SERVICE_UNAVAILABLE:
      return "Service is temporarily unavailable. Please try again later.";
    default:
      return error.message || "An unexpected error occurred.";
  }
};

/**
 * Handle async operation with error handling
 * @param {Function} asyncFunction - Async function to execute
 * @returns {Array} [data, error]
 */
export const handleAsync = async (asyncFunction) => {
  try {
    const data = await asyncFunction();
    return [data, null];
  } catch (error) {
    return [null, error];
  }
};
