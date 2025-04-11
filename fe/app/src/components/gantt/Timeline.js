import React from 'react';

import './Timeline.css';
import './Common.css';



const TimelineHeader = ({startDate, endDate, width, headerResolution}) => {

  console.log(startDate, endDate, width, headerResolution);
  
  // console.log(startDate.getMonth);

  // // if (headerResolution === 'month') {
  //   const months = [];
  //   for (let date = startDate; date <= endDate; date.setMonth(date.getMonth() + 1)) {
  //     months.push(new Date(date));
  //   }
  // // }

  // console.log(months);

  return (
    <div className="timeline-header">
      {/* <div className="timeline-header-item">Header 1</div> */}
    </div>
  );
};



const TimelineBody = ({startDate, endDate, width, rowHeight, data}) => {

  const startDateUnix = startDate.getTime();
  const endDateUnix = endDate.getTime();
  const totalDuration = endDateUnix - startDateUnix;

  // Normalize the date to the width of the timeline
  const normalizeToWidth = (date) => {
    return ((date.getTime() - startDateUnix) / totalDuration) * width;
  };

  // console.log(data);

  

  return (
    <div className="timeline-body">
      {data.map((item, index) => {
        const itemStartX = normalizeToWidth(new Date(item.start));
        const itemEndX = normalizeToWidth(new Date(item.finish));
        const itemWidth = itemEndX - itemStartX;

        console.log(item, itemStartX, itemEndX, itemWidth);

        return (
          <React.Fragment key={index}>
            <div 
              className="gantt-row"
              style={{
                // top: `${index * rowHeight}px`,
                height: `${rowHeight}px`,
              }}
            >
              <div
                className="timeline-item"
                style={{
                  left: `${itemStartX}px`,
                  width: `${itemWidth}px`,
                  height: `100%`,
                  backgroundColor: item.color || 'rgb(85, 155, 241)',
                }}
              >
                {/* {item.name} */}
              </div>
            </div>
          </React.Fragment>
        );
      })}
    </div>
  );
};


const Timeline = ({startDate, endDate, rowHeight, width, headerResolution, data}) => {


  return (
    <div className="timeline-container">
      <div 
        className="timeline" 
        style={{ 
          width: width,
        }}
      >

        <TimelineHeader 
          startDate={startDate} 
          endDate={endDate} 
          width={width} 
          headerResolution={headerResolution} 
        />

        <TimelineBody 
          startDate={startDate} 
          endDate={endDate} 
          width={width} 
          rowHeight={rowHeight} 
          data={data} 
        />

      </div>
    </div>
  );
};

export default Timeline;
