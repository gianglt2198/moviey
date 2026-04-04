import React from "react";
import VideoPlayer from "../../pages/VideoPlayer";
import "../../styles/components/VideoPlayerModal.css";

function VideoPlayerModal({ movie, src, onClose, onProgress }) {
  return (
    <div className="player-modal">
      <div className="player-wrapper">
        <button className="close-btn" onClick={onClose} title="Close player">
          ✕
        </button>
        <VideoPlayer src={src} onProgress={onProgress} />
        {movie && (
          <div className="movie-details">
            <h2>{movie.title}</h2>
            <p>{movie.description}</p>
            {movie.genre && <span className="badge">{movie.genre}</span>}
          </div>
        )}
      </div>
    </div>
  );
}

export default VideoPlayerModal;
