import { useState, useCallback } from "react";
import { authService } from "../services/api";

export const useAuth = () => {
  const [isLoggedIn, setIsLoggedIn] = useState(!!localStorage.getItem("token"));
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const login = useCallback(async (email, password) => {
    setLoading(true);
    setError(null);

    try {
      const { data } = await authService.login(email, password);
      localStorage.setItem("token", data.token);
      setIsLoggedIn(true);
      return data;
    } catch (err) {
      setError(err.response?.data?.message || "Login failed");
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  const logout = useCallback(() => {
    authService.logout();
    setIsLoggedIn(false);
  }, []);

  return { isLoggedIn, login, logout, loading, error };
};
