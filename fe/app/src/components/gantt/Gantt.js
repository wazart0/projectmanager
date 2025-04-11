import React, { useRef, useEffect, useState } from 'react';
import './Gantt.css';

import Timeline from './Timeline';
import ListOfTasks from './ListOfTasks';
// import config from '../config.js';
// import { DateTime } from 'luxon';



function Gantt({data}) {
  const dividerRef = useRef(null);
  const leftPanelRef = useRef(null);
  const rightPanelRef = useRef(null);
  const [dividerHeight, setDividerHeight] = useState(0);

  const showColumns = ['id', 'task_name', 'start', 'finish'];

  useEffect(() => {
    const divider = dividerRef.current;
    const leftPanel = leftPanelRef.current;
    const rightPanel = rightPanelRef.current;

    let isResizing = false;
    let lastX = 0;
    let leftWidth = 0;

    const startResizing = (clientX) => {
      isResizing = true;
      lastX = clientX;
      leftWidth = leftPanel.getBoundingClientRect().width;
    };

    const stopResizing = () => {
      isResizing = false;
    };

    const resize = (clientX) => {
      if (!isResizing) return;
      const dx = clientX - lastX;
      const newLeftWidth = leftWidth + dx;
      leftPanel.style.width = `${newLeftWidth}px`;
      rightPanel.style.width = `calc(100% - ${newLeftWidth}px)`;
    };

    const handleMouseDown = (e) => {
      startResizing(e.clientX);
    };

    const handleMouseMove = (e) => {
      resize(e.clientX);
    };

    const handleMouseUp = () => {
      stopResizing();
    };

    const handleTouchStart = (e) => {
      startResizing(e.touches[0].clientX);
    };

    const handleTouchMove = (e) => {
      resize(e.touches[0].clientX);
    };

    const handleTouchEnd = () => {
      stopResizing();
    };

    divider.addEventListener('mousedown', handleMouseDown);
    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);

    divider.addEventListener('touchstart', handleTouchStart);
    document.addEventListener('touchmove', handleTouchMove);
    document.addEventListener('touchend', handleTouchEnd);

    return () => {
      divider.removeEventListener('mousedown', handleMouseDown);
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);

      divider.removeEventListener('touchstart', handleTouchStart);
      document.removeEventListener('touchmove', handleTouchMove);
      document.removeEventListener('touchend', handleTouchEnd);
    };
  }, []);

  useEffect(() => {
    // Action to be performed on webpage load
    console.log('Webpage loaded');
    // Add your desired actions here
    const ganttContainer = document.querySelector('.gantt-container');
    if (ganttContainer) {
      const leftPanel = leftPanelRef.current;
      const rightPanel = rightPanelRef.current;
      const ganttContainerWidth = ganttContainer.getBoundingClientRect().width;
      const leftPanelWidth = ganttContainerWidth/2;  // TODO: Read from variables
      console.log('Gantt container width:', ganttContainerWidth);
      leftPanel.style.width = `${leftPanelWidth}px`;
      rightPanel.style.width = `calc(100% - ${leftPanelWidth}px)`;
    }
  }, []); // Empty dependency array ensures the effect runs only once on mount

  useEffect(() => {
    const updateDividerHeight = () => {
      const listContainer = document.querySelector('.list-of-tasks-container');
      if (listContainer) {
        setDividerHeight(listContainer.offsetHeight);
      }
    };

    updateDividerHeight();
    // Update height when window is resized
    window.addEventListener('resize', updateDividerHeight);
    
    return () => {
      window.removeEventListener('resize', updateDividerHeight);
    };
  }, [data]); // Re-run when data changes as it might affect list height

  return (
    <div className="gantt-container">
      <div ref={leftPanelRef} className="gantt-panel left-panel">
        {/* Content for the left panel */}
        <ListOfTasks 
          rowHeight={30}
          data={data}
          showColumns={showColumns}
        />
      </div>
      <div ref={dividerRef} className="divider" style={{height: dividerHeight}}></div>
      <div ref={rightPanelRef} className="gantt-panel right-panel">
        {/* Content for the right panel */}
        <Timeline 
          startDate={new Date('2023-06-01T09:00:00')}
          endDate={new Date('2023-06-27T13:00:00')}
          rowHeight={30}
          width={3000}
          headerResolution={'month'}
          data={data}
        />
      </div>
    </div>
  );
}

export default Gantt;
