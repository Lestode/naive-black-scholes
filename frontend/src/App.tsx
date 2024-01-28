import React from 'react';
import "./App.css";
import SearchBar from './components/SearchBar';
import './css/global.css';


function App() {
  return (
    <div className="App">
      <header className="App-header">
        <SearchBar />
        {/* Other components */}
      </header>
    </div>
  );
}

export default App;
