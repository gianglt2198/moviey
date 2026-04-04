import axios from "axios";
import envConfig from "../../env.config";

const apiClient = axios.create({
  baseURL: envConfig.API_BASE_URL,
  timeout: envConfig.TIMEOUT,
  headers: {
    "Content-Type": "application/json",
  },
});

// Request Interceptor - Add Authorization Token
apiClient.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem("token");
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    if (envConfig.DEBUG) {
      console.log("[API Request]", config.method.toUpperCase(), config.url);
    }
    return config;
  },
  (error) => {
    console.error("[API Request Error]", error);
    return Promise.reject(error);
  },
);

// Response Interceptor - Handle Errors
apiClient.interceptors.response.use(
  (response) => {
    if (envConfig.DEBUG) {
      console.log("[API Response]", response.status, response.config.url);
    }
    return response;
  },
  (error) => {
    // Handle 401 Unauthorized
    if (error.response?.status === 401) {
      console.warn("[API Error] 401 Unauthorized - Redirecting to login");
      localStorage.removeItem("token");
      window.location.href = "/";
    }

    // Handle 403 Forbidden
    if (error.response?.status === 403) {
      console.error("[API Error] 403 Forbidden");
    }

    // Handle Server Errors
    if (error.response?.status >= 500) {
      console.error("[API Error] Server Error:", error.response.status);
    }

    return Promise.reject(error);
  },
);

export default apiClient;
