import React, { useState } from "react";
import {
  TextField,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  ListItemText,
  Paper,
} from "@mui/material";
import SearchIcon from "@mui/icons-material/Search";
import { fetchBestMatches } from "../apiService";

const SearchBar = () => {
  const [searchTerm, setSearchTerm] = useState("");
  const [searchResults, setSearchResults] = useState<string[]>([]);

  const handleSearchChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setSearchTerm(e.target.value);
  };

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (searchTerm.trim() !== "") {
      try {
        const results = await fetchBestMatches(searchTerm);
        setSearchResults(results);
      } catch (error) {
        console.error("Failed to fetch search results:", error);
        setSearchResults([]);
      }
    } else {
      setSearchResults([]);
    }
    console.log("Searching for:", searchTerm);
  };

  const handleItemClick = () => {
    // Perform your action on item click here
    // For example, navigate to a different page or display item details
    console.log("Item clicked");
    // history.push(`/details/${item}`); // Uncomment if you're using routing
  };

  return (
    <div>
      <form
        onSubmit={handleSubmit}
        style={{ display: "flex", alignItems: "center" }}
      >
        <TextField
          label="Search for a stock symbol:"
          variant="outlined"
          value={searchTerm}
          onChange={handleSearchChange}
          style={{ flexGrow: 1 }}
        />
        <IconButton type="submit" aria-label="search">
          <SearchIcon />
        </IconButton>
      </form>
      {searchResults.length > 0 && (
        <Paper
          sx={{ mt: 2, overflow: "hidden", borderRadius: "8px", elevation: 3 }}
        >
          <List component="nav" aria-label="search results">
            {searchResults.map((result, index) => (
              <ListItem button key={index} onClick={handleItemClick}>
                <ListItemText primary={result} />
              </ListItem>
            ))}
          </List>
        </Paper>
      )}
    </div>
  );
};

function sortMonths(months: number[]): number[] {
  const currentMonth = new Date().getMonth() + 1; // JavaScript months are 0-based
  return months.sort((a, b) => {
    const adjustedA =
      a >= currentMonth ? a - currentMonth : a + 12 - currentMonth;
    const adjustedB =
      b >= currentMonth ? b - currentMonth : b + 12 - currentMonth;
    return adjustedA - adjustedB;
  });
}
export default SearchBar;
