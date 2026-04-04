export const API_ENDPOINTS = {
  AUTH: {
    LOGIN: "/auth/login",
    LOGOUT: "/auth/logout",
    REGISTER: "/auth/register",
  },
  MOVIES: {
    LIST: "/movies",
    SEARCH: "/movies/search",
    DETAIL: (id) => `/movies/${id}`,
    UPLOAD: "/movies/upload",
  },
  FAVORITES: {
    LIST: "/favorites",
    TOGGLE: "/favorites/toggle",
  },
  WATCH_HISTORY: {
    LIST: "/watch-history",
    SAVE: "/watch-progress",
    SAVE_ENHANCED: "/watch-history/save-enhanced",
  },
  RECOMMENDATIONS: {
    GENERATE: (userId) => `/recommendations/generate/${userId}`,
    GET: (userId) => `/recommendations/${userId}`,
    SIMILAR: (movieId) => `/recommendations/similar/${movieId}`,
    FEEDBACK: "/recommendations/feedback",
  },
  ANALYTICS: {
    COMPLETION_BY_GENRE: "/analytics/completion-by-genre",
    WATCH_PATTERNS: "/analytics/watch-patterns",
    DATA_QUALITY: "/analytics/data-quality",
    USER_SEGMENT: "/analytics/user/segment",
  },
};

export const HTTP_STATUS = {
  OK: 200,
  CREATED: 201,
  BAD_REQUEST: 400,
  UNAUTHORIZED: 401,
  FORBIDDEN: 403,
  NOT_FOUND: 404,
  SERVER_ERROR: 500,
  SERVICE_UNAVAILABLE: 503,
};

export const GENRES = [
  "Action",
  "Drama",
  "Comedy",
  "Horror",
  "Thriller",
  "Sci-Fi",
  "Romance",
  "Animation",
  "Documentary",
];

export const SORT_OPTIONS = [
  { value: "recent", label: "Recently Added" },
  { value: "rating", label: "Top Rated" },
  { value: "title", label: "Title A-Z" },
];
