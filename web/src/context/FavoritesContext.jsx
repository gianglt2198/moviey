import React, { createContext, useContext } from "react";
import { useFavorites } from "../hooks";

const FavoritesContext = createContext();

export const FavoritesProvider = ({ children }) => {
  const favorites = useFavorites();

  return (
    <FavoritesContext.Provider value={favorites}>
      {children}
    </FavoritesContext.Provider>
  );
};

export const useFavoritesContext = () => {
  const context = useContext(FavoritesContext);
  if (!context) {
    throw new Error(
      "useFavoritesContext must be used within FavoritesProvider",
    );
  }
  return context;
};
