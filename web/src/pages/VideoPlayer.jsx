import React, { useEffect, useRef } from 'react';
import videojs from 'video.js';
import 'video.js/dist/video-js.css';

const VideoPlayer = ({ src, onProgress }) => {
  const videoRef = useRef(null);
  const playerRef = useRef(null);

  useEffect(() => {
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

      // Track progress every 10 seconds
      player.on('timeupdate', () => {
        const currentTime = Math.floor(player.currentTime());
        if (currentTime % 10 === 0 && onProgress) {
          onProgress(currentTime);
        }
      });
    } else {
      const player = playerRef.current;
      player.src({ src, type: 'application/x-mpegURL' });
    }
  }, [src, onProgress]);

  useEffect(() => {
    const player = playerRef.current;
    return () => {
      if (player) {
        player.dispose();
        playerRef.current = null;
      }
    };
  }, [playerRef]);

  return <div data-vjs-player ref={videoRef} style={{ width: '100%' }} />;
};

export default VideoPlayer;
