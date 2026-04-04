import React, { useEffect } from "react";
import { AppProvider, useAuthContext } from "./context";
import { ErrorBoundary } from "./components";
import LoginPage from "./pages/LoginPage";
import HomePage from "./pages/HomePage";
import "./App.css";

function AppContent() {
  const { isLoggedIn, loading } = useAuthContext();

  if (loading) {
    return (
      <div className="flex-center min-h-screen">
        <div className="animate-spin text-4xl">⏳</div>
      </div>
    );
  }

  return isLoggedIn ? <HomePage /> : <LoginPage />;
}

export default function App() {
  return (
    <ErrorBoundary>
      <AppProvider>
        <AppContent />
      </AppProvider>
    </ErrorBoundary>
  );
}
