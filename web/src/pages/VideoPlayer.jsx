import React, { useEffect, useRef } from 'react';
import videojs from 'video.js';
import 'video.js/dist/video-js.css';

const VideoPlayer = ({ src }) => {
  const videoRef = useRef(null);
  const playerRef = useRef(null);

  useEffect(() => {
    console.log('Initializing Video Player with src:', src);
    // Initialize Video.js player
    if (!playerRef.current) {
      const placeholderEl = videoRef.current;
      const videoElement = placeholderEl.appendChild(
        document.createElement('video-js'),
      );

      const player = (playerRef.current = videojs(videoElement, {
        autoplay: false,
        controls: true,
        responsive: true,
        fluid: true,
        sources: [{ src, type: 'application/x-mpegURL' }],
      }));
    } else {
      // If src changes, update the player
      const player = playerRef.current;
      player.src({ src, type: 'application/x-mpegURL' });
    }
  }, [src]);

  // Dispose the player on unmount
  useEffect(() => {
    const player = playerRef.current;
    return () => {
      if (player) {
        player.dispose();
        playerRef.current = null;
      }
    };
  }, [playerRef]);

  return (
    <div data-vjs-player ref={videoRef}>
      {/* <video ref={videoRef} className="video-js vjs-big-play-centered" /> */}
    </div>
  );
};

export default VideoPlayer;
