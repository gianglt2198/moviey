import React, { createContext, useContext } from "react";
import { useMovies, useSearch } from "../hooks";

const MoviesContext = createContext();

export const MoviesProvider = ({ children }) => {
  const movies = useMovies();
  const search = useSearch();

  const value = {
    movies,
    search,
  };

  return (
    <MoviesContext.Provider value={value}>{children}</MoviesContext.Provider>
  );
};

export const useMoviesContext = () => {
  const context = useContext(MoviesContext);
  if (!context) {
    throw new Error("useMoviesContext must be used within MoviesProvider");
  }
  return context;
};
