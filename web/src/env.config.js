const getEnvConfig = () => {
  const env = import.meta.env.MODE || "development";

  const config = {
    development: {
      API_BASE_URL: "http://localhost:3000/api",
      DEBUG: true,
      TIMEOUT: 10000,
    },
    production: {
      API_BASE_URL: "https://api.moviey.com/api",
      DEBUG: false,
      TIMEOUT: 10000,
    },
  };

  return config[env] || config.development;
};

export default getEnvConfig();
