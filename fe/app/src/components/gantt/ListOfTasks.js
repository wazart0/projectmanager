import React from 'react';

import './ListOfTasks.css';
import './Common.css';



// const ListOfTasksHeader = ({data}) => {

//   console.log(data);
  
//   // console.log(startDate.getMonth);

//   // // if (headerResolution === 'month') {
//   //   const months = [];
//   //   for (let date = startDate; date <= endDate; date.setMonth(date.getMonth() + 1)) {
//   //     months.push(new Date(date));
//   //   }
//   // // }

//   // console.log(months);

//   return (
//   );
// };



const ListOfTasksBody = ({rowHeight, data, showColumns}) => {

  

  return (
    <div className="list-of-tasks-body">
      {showColumns.map((column, index) => {
        return (
          <React.Fragment key={index}>

            <div className="list-of-tasks-column" key={index}>

              {/* <div className="list-of-tasks-header">
                <div className="timeline-header-cell">{column}</div>
              </div> */}
              
              {data.map((item, index) => {
                return (
                  <React.Fragment key={index}>
                    <div
                      className="gantt-row"
                      style={{
                        // top: `${index * rowHeight}px`,
                        height: `${rowHeight}px`,                
                      }}
                    >
                      <div className="list-of-tasks-cell">
                        {item[column]}
                      </div>
                    </div>
                  </React.Fragment>
                );
              })}

            </div>
          </React.Fragment>
        );
      })}

      <div className="list-of-tasks-column-empty">
        {data.map((item, index) => {
          return (
            <React.Fragment key={index}>
              <div
                className="gantt-row"
                style={{
                  // top: `${index * rowHeight}px`,
                    height: `${rowHeight}px`,                
                  }}
              >
                <div className="list-of-tasks-cell" key={index}>
                </div>
              </div>
            </React.Fragment>
          );
        })}
      </div>
    </div>
  );
};


const ListOfTasks = ({rowHeight, data, showColumns}) => {


  return (
    <div className="list-of-tasks-container">
      <div 
        className="list-of-tasks" 
      >
        <ListOfTasksBody 
          rowHeight={rowHeight} 
          data={data} 
          showColumns={showColumns}
        />

      </div>
    </div>
  );
};

export default ListOfTasks;
