import React, { createContext, useContext } from "react";
import { AuthProvider } from "./AuthContext";
import { MoviesProvider } from "./MoviesContext";
import { FavoritesProvider } from "./FavoritesContext";
import { WatchHistoryProvider } from "./WatchHistoryContext";

const AppContext = createContext();

/**
 * Main app provider that wraps all context providers
 */
export const AppProvider = ({ children }) => {
  return (
    <AuthProvider>
      <MoviesProvider>
        <FavoritesProvider>
          <WatchHistoryProvider>{children}</WatchHistoryProvider>
        </FavoritesProvider>
      </MoviesProvider>
    </AuthProvider>
  );
};

/**
 * Hook to use app context
 */
export const useAppContext = () => {
  const context = useContext(AppContext);
  if (!context) {
    throw new Error("useAppContext must be used within AppProvider");
  }
  return context;
};
