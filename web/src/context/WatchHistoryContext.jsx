import React, { createContext, useContext } from "react";
import { useWatchHistory } from "../hooks";

const WatchHistoryContext = createContext();

export const WatchHistoryProvider = ({ children }) => {
  const watchHistory = useWatchHistory();

  return (
    <WatchHistoryContext.Provider value={watchHistory}>
      {children}
    </WatchHistoryContext.Provider>
  );
};

export const useWatchHistoryContext = () => {
  const context = useContext(WatchHistoryContext);
  if (!context) {
    throw new Error(
      "useWatchHistoryContext must be used within WatchHistoryProvider",
    );
  }
  return context;
};
