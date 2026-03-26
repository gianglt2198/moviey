import React, { useState } from 'react';
import axios from 'axios';
import '../styles/SearchBar.css';

function SearchBar({ onSearch }) {
  const [searchTerm, setSearchTerm] = useState('');
  const [genre, setGenre] = useState('');
  const [sortBy, setSortBy] = useState('recent');

  const handleSearch = async () => {
    try {
      const res = await axios.get('http://localhost:3000/api/movies/search', {
        params: {
          q: searchTerm || undefined,
          genre: genre || undefined,
          sort: sortBy,
        },
      });
      onSearch(res.data);
    } catch (err) {
      console.error('Search failed:', err);
    }
  };

  const handleKeyPress = (e) => {
    if (e.key === 'Enter') {
      handleSearch();
    }
  };

  return (
    <div className="search-bar">
      <div className="search-controls">
        <input
          type="text"
          placeholder="🔍 Search movies..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          onKeyPress={handleKeyPress}
          className="search-input"
        />

        <select
          value={genre}
          onChange={(e) => setGenre(e.target.value)}
          className="filter-select"
        >
          <option value="">All Genres</option>
          <option value="Action">Action</option>
          <option value="Drama">Drama</option>
          <option value="Comedy">Comedy</option>
          <option value="Horror">Horror</option>
          <option value="Thriller">Thriller</option>
          <option value="Sci-Fi">Sci-Fi</option>
        </select>

        <select
          value={sortBy}
          onChange={(e) => setSortBy(e.target.value)}
          className="filter-select"
        >
          <option value="recent">Recently Added</option>
          <option value="rating">Top Rated</option>
          <option value="title">Title A-Z</option>
        </select>

        <button onClick={handleSearch} className="search-btn">
          Search
        </button>
      </div>
    </div>
  );
}

export default SearchBar;
