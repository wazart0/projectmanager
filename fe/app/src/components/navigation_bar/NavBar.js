import React from 'react';
// import { Link } from 'react-router-dom';

import './NavBar.css';

const NavBar = () => {
  return (
    <nav>
      <ul>
        <li>
          {/* <Link to="/">Home</Link> */}
          Home
        </li>
        <li>
          {/* <Link to="/about">About</Link> */}
          About
        </li>
        <li>
          Contact
          {/* <Link to="/contact">Contact</Link> */}
        </li>
      </ul>
    </nav>
  );
};

export default NavBar;
