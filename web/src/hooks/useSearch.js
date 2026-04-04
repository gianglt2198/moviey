import { useState, useCallback, useMemo } from "react";
import { movieService } from "../services/api";

export const useSearch = () => {
  const [searchResults, setSearchResults] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [searchParams, setSearchParams] = useState({
    query: "",
    genre: "",
    sort: "recent",
  });

  /**
   * Perform search with parameters
   */
  const search = useCallback(
    async (params = searchParams) => {
      setLoading(true);
      setError(null);
      setSearchParams(params);

      try {
        const { data } = await movieService.searchMovies({
          q: params.query || undefined,
          genre: params.genre || undefined,
          sort: params.sort,
        });
        setSearchResults(data);
        return data;
      } catch (err) {
        setError(err);
        console.error("Search failed:", err);
        throw err;
      } finally {
        setLoading(false);
      }
    },
    [searchParams],
  );

  /**
   * Clear search results
   */
  const clearSearch = useCallback(() => {
    setSearchResults([]);
    setSearchParams({
      query: "",
      genre: "",
      sort: "recent",
    });
  }, []);

  /**
   * Update search parameters
   */
  const updateSearchParams = useCallback((newParams) => {
    setSearchParams((prev) => ({
      ...prev,
      ...newParams,
    }));
  }, []);

  return {
    results: searchResults,
    loading,
    error,
    params: searchParams,
    search,
    clearSearch,
    updateSearchParams,
  };
};
