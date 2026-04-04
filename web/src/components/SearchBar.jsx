import React, { useState } from "react";
import { GENRES, SORT_OPTIONS } from "../constants/api";
import "../styles/components/SearchBar.css";

function SearchBar({ onSearch, params = {} }) {
  const [searchTerm, setSearchTerm] = useState(params.query || "");
  const [genre, setGenre] = useState(params.genre || "");
  const [sortBy, setSortBy] = useState(params.sort || "recent");

  const handleSearch = async () => {
    await onSearch({
      query: searchTerm,
      genre,
      sort: sortBy,
    });
  };

  const handleKeyPress = (e) => {
    if (e.key === "Enter") {
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
          {GENRES.map((g) => (
            <option key={g} value={g}>
              {g}
            </option>
          ))}
        </select>

        <select
          value={sortBy}
          onChange={(e) => setSortBy(e.target.value)}
          className="filter-select"
        >
          {SORT_OPTIONS.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>

        <button onClick={handleSearch} className="search-btn">
          Search
        </button>
      </div>
    </div>
  );
}

export default SearchBar;
