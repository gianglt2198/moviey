import React from "react";
import "./Loader.css";

/**
 * Reusable Loader Component
 * @param {string} size - Loader size (sm, md, lg)
 * @param {string} text - Loading text
 * @param {boolean} fullscreen - Fullscreen loader
 */
export const Loader = ({ size = "md", text, fullscreen = false }) => {
  const LoaderContent = () => (
    <div className={`loader loader-${size}`}>
      <div className="loader-spinner" />
      {text && <p className="loader-text">{text}</p>}
    </div>
  );

  if (fullscreen) {
    return (
      <div className="loader-fullscreen">
        <LoaderContent />
      </div>
    );
  }

  return <LoaderContent />;
};
