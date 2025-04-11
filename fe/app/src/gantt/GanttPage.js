import React, { useRef, useEffect, useState } from 'react';
import './GanttPage.css';
import NavBar from '../components/navigation_bar/NavBar.js';
import Gantt from '../components/gantt/Gantt.js';
import Footer from '../components/footer/Footer.js';
// import config from '../config.js';
// import { DateTime } from 'luxon';



function GanttPage() {
  const [isLoading, setIsLoading] = useState(true);
  const [data, setData] = useState(null);

  useEffect(() => {
    fetch('http://0.0.0.0:22000/test_data.json')
      .then(response => response.json())
      .then(jsonData => {
        setData(jsonData);
        setIsLoading(false);
      })
      .catch(error => {
        console.error('Error fetching data:', error);
        setIsLoading(false);
      });
  }, []);

  return (
    <div className="gantt-page">
      <NavBar />
      {isLoading ? (
        <div>Loading...</div>
      ) : (
        <Gantt data={data} />
      )}
      <Footer />
    </div>
  );
}

export default GanttPage;
