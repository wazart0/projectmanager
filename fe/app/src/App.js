import './App.css';
import LoginPage from './login/LoginPage';
import GanttPage from './gantt/GanttPage';
import React from 'react';
import Navbar from './components/navigation_bar/NavBar';


// Add auth check function (you'll need to implement this based on your auth system)
const checkAuthStatus = () => {
  // Example implementation - modify according to your auth system
  return localStorage.getItem('authToken') !== null;
};


function App() {
  if (!checkAuthStatus()) 
    return (
      <div className="App">
        <LoginPage />
      </div>
    )
  return (
    <div className="App">
      <GanttPage />
    </div>
  );
}

export default App;
