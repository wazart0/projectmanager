import pandas as pd
import os
from dotenv import dotenv_values


APP_PATH = '/app'



# Using odfpy to read ODS files directly
import odf.opendocument
from odf.table import Table, TableRow, TableCell
from odf.text import P

file_path = f"{APP_PATH}/local/tmp/eic2025summary-people.ods"

spreadsheet = odf.opendocument.load(file_path)

def get_team_cost_report(spreadsheet):
    # Get all tables (sheets) in the document
    tables = spreadsheet.getElementsByType(Table)

    # Find the specific table by name
    team_cost_report = None
    for table in tables:
        if table.getAttribute("name") == "Team_total_cost":
            team_cost_report = table
            break

    # Extract data from the table
    if team_cost_report:
        rows = team_cost_report.getElementsByType(TableRow)
        
        # Start from row 3 (index 2 since it's 0-indexed)
        data_rows = rows[2:]
        
        # Process each row
        extracted_data = []
        for row in data_rows:
            cells = row.getElementsByType(TableCell)
            row_data = []
            
            # Process each cell in the row
            for cell in cells:
                # Get the cell content
                cell_content = ""
                
                # Check if the cell has any P elements (text paragraphs)
                paragraphs = cell.getElementsByType(P)
                if paragraphs:
                    # Join all paragraphs in the cell
                    cell_content = " ".join([p.firstChild.data if p.firstChild else "" for p in paragraphs])
                
                # Handle repeated cells (if a cell spans multiple columns)
                repeat_count = cell.getAttribute("numbercolumnsrepeated")
                if repeat_count:
                    # Add the same content multiple times based on repeat count
                    row_data.extend([cell_content] * int(repeat_count))
                else:
                    row_data.append(cell_content)
            
            # Only take columns A to AH (first 34 columns)
            # A=0, B=1, ..., Z=25, AA=26, ..., AH=33
            if row_data:
                row_data = row_data[:34]  # Limit to columns A through AH
                extracted_data.append(row_data)
        
        # Convert to DataFrame if needed
        if extracted_data:
            columns = ['id', 'first_name', 'last_name', 'position', 'comment'] + [f'{i}' for i in range(1, 30)]

            df = pd.DataFrame(extracted_data, columns=columns)
            df = df[df['id'] != '']
            print(f"Extracted {len(df)} rows of data")
            return df
    else:
        print("Table 'Team_total_cost' not found in the document")


df_costs_per_person_per_month = get_team_cost_report(spreadsheet)


def get_time_report(spreadsheet, team_ids, number_of_months):

    def extract_cells_A_to_S(row):
        """
        Extract cell values from columns A to S from a given row.
        
        Args:
            row: An ODS TableRow object
        
        Returns:
            List of cell values from columns A to S (indices 0 to 18)
        """
        cells = row.getElementsByType(TableCell)
        row_data = []
        
        # Process each cell in the row
        for cell in cells:
            # Get the cell content
            cell_content = ""
            
            # Check if the cell has any P elements (text paragraphs)
            paragraphs = cell.getElementsByType(P)
            if paragraphs:
                # Join all paragraphs in the cell
                cell_content = " ".join([p.firstChild.data if p.firstChild else "" for p in paragraphs])
            
            # Handle repeated cells (if a cell spans multiple columns)
            repeat_count = cell.getAttribute("numbercolumnsrepeated")
            if repeat_count:
                # Add the same content multiple times based on repeat count
                row_data.extend([cell_content] * int(repeat_count))
            else:
                row_data.append(cell_content)
        
        # Return only columns A to S (indices 0 to 18)
        # If row_data doesn't have enough elements, return as many as possible
        if len(row_data) <= 0:
            return []
        else:
            return row_data[0:min(19, len(row_data))]

    row_offset = 7
    tables = spreadsheet.getElementsByType(Table)
    time_report = []

    columns = ['month', 'T1.1', 'T1.2', 'T2.1', 'T2.2', 'T2.3', 'T3.1', 'T3.2', 'T3.3', 'T4.1', 'T4.2', 'T5.1', 'T5.2', 'T5.3', 'T6.1', 'T6.2', 'T6.3', 'T6.4', 'T6.5']
    
    for table in tables:
        sheet_name = table.getAttribute("name")
        if not sheet_name in team_ids:
            continue

        # Get row 8 (with offset of 9, this is index 8+9-1 = 16)
        rows = table.getElementsByType(TableRow)
        if len(rows) <= row_offset + number_of_months + 1:
            print(f"Table {sheet_name} doesn't have enough rows to extract row 8")
            continue
            
        # Convert row to list of values
        for row in rows[row_offset:row_offset+number_of_months+1]:
        
            row_data = extract_cells_A_to_S(row)

            for i in range(1, len(row_data)):
                if row_data[i] == '':
                    continue

                time_report.append({
                    'user_id': sheet_name,
                    'task_name': columns[i],
                    'timespent_h': row_data[i],
                    'month': row_data[0]
                })

    df_time_report = pd.DataFrame(time_report, columns=['user_id', 'task_name', 'timespent_h', 'month'])
    df_time_report['timespent_h'] = df_time_report['timespent_h'].astype(float)
    df_time_report['month'] = df_time_report['month'].astype(int)

    return df_time_report

df_time_report = get_time_report(spreadsheet, df_costs_per_person_per_month.id.to_list(), 29)


# Transform the cost report from wide to long format
def transform_cost_report_to_long(df):
    # Identify the ID columns and value columns
    id_columns = ['id', 'first_name', 'last_name', 'position', 'comment']
    value_columns = [str(i) for i in range(1, 30)]
    
    # Transform from wide to long format
    df_long = pd.melt(
        df,
        id_vars=id_columns,
        value_vars=value_columns,
        var_name='month',
        value_name='cost'
    )
    
    # Convert data types
    df_long['month'] = df_long['month'].astype(int)
    df_long['cost'] = pd.to_numeric(df_long['cost'], errors='coerce')
    
    # Remove rows with null or zero costs
    df_long = df_long.dropna(subset=['cost'])
    df_long = df_long[df_long['cost'] != 0]
    
    return df_long

# Transform the cost report
df_costs_long = transform_cost_report_to_long(df_costs_per_person_per_month)

print(f"Transformed cost report: {len(df_costs_long)} rows")


