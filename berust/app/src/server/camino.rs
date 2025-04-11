use std::fs;
use polars::prelude::*;
use spreadsheet_ods::*;




const APP_PATH: &str = "/app";
const PROJECT_INFO: &str = "/app/local/tmp/project_info.json";
const LIST_OF_TASKS: &str = "/app/local/tmp/list_of_tasks.json";

const ROW_OFFSET: i32 = 8;




pub fn df_to_str_json(df: &mut DataFrame) -> Result<String, String> {
    // Create a buffer
    let mut buffer = Vec::new();

    // Convert DataFrame to JSON format where each row is an object and write to buffer
    JsonWriter::new(&mut buffer)
        .with_json_format(JsonFormat::Json)
        .finish(df)
        .map_err(|e| String::from(format!("Failed to write JSON: {e}")))?;
    
    // Convert buffer to string
    String::from_utf8(buffer)
        .map_err(|e| String::from(format!("Failed to convert buffer to string: {e}")))
}



pub fn ods_cell_to_string(cell: CellContent) -> String {
    match cell.value() {
        Value::Empty => "".to_string(),
        Value::Text(s) => s.to_string(),
        Value::Number(n) => n.to_string(),
        Value::DateTime(dt) => dt.to_string(),
        Value::Boolean(b) => b.to_string(),
        _ => format!("{:?}", cell.value()),
    }
}


fn read_json_file(path: &str) -> Result<serde_json::Value, String> {
    let data = fs::read_to_string(path)
        .unwrap_or_else(|e| format!("Failed to read project info file: {e}"));
    
    serde_json::from_str(&data)
        .map_err(|e| format!("Invalid JSON in project info file: {e}"))
}


fn read_ods_file() -> Result<WorkBook, String> {    
    read_ods(format!("{APP_PATH}/local/tmp/eic2025summary-people.ods"))
        .map_err(|e| format!("Failed to read ODS file: {e}"))
}


fn get_sheet<'a>(wb: &'a WorkBook, name: &str) -> Result<&'a Sheet, String> {
    Ok(wb.sheet(wb.sheet_idx(name)
        .ok_or(format!("Cannot find sheet with name {name}"))?))
}


fn read_json_to_df(path: &str) -> Result<DataFrame, String> {
    JsonReader::new(
        std::fs::File::open(path)
            .map_err(|e| format!("Error when opening file: {e}"))?
    )
    .finish()
    .map_err(|e| format!("Error when reading file: {e}"))  
}



pub fn get_project_info_df() -> Result<DataFrame, String> {
    read_json_to_df(&PROJECT_INFO)
}


pub fn get_tasks_df() -> Result<DataFrame, String> {
    read_json_to_df(&LIST_OF_TASKS)
}

pub fn get_other_costs_df() -> Result<LazyFrame, String> {
    Ok(LazyCsvReader::new(format!("{APP_PATH}/local/tmp/Team - Faktury.csv"))
        .with_has_header(true)
        .finish()
        .map_err(|e| format!("Error reading resources file: {e}"))?
        .select([
            col("Dokument").cast(DataType::Date).alias("document_number"),
            col("Data dok.").alias("date"), 
            col("Treść").alias("description"), 
            col("Suma").cast(DataType::Float64).alias("cost"), 
            col("Task").alias("task_name"), 
            col("Cost type").alias("cost_type"),
            lit(String::from("PLN")).alias("currency")
        ])
        .lazy()
        .filter(col("date").is_not_null()))
}


pub fn get_time_report_df() -> Result<DataFrame, String> {
    let wb = read_ods_file()?;
    
    let columns = vec!["month", "T1.1", "T1.2", "T2.1", "T2.2", "T2.3", "T3.1", "T3.2", "T3.3", "T4.1", "T4.2", "T5.1", "T5.2", "T5.3", "T6.1", "T6.2", "T6.3", "T6.4", "T6.5"];

    let relevant_sheets: Vec<_> = wb.iter_sheets()
        .filter(|sheet| sheet.name().parse::<i32>().is_ok() )
        .collect();

    let mut user_id_column: Vec<String> = vec![];
    let mut task_name_column: Vec<String> = vec![];
    let mut timespent_column: Vec<String> = vec![];
    let mut month_column: Vec<String> = vec![];
    
    for sheet in relevant_sheets {
        let user_id = sheet.name().to_string();
        
        // Iterate through months (rows 9-37)
        for row_idx in ROW_OFFSET..=(ROW_OFFSET + 29) {
            let month_num = (row_idx - ROW_OFFSET + 1).to_string();
            
            // Iterate through tasks (columns B-S, indices 1-18)
            for col_idx in 1..=18 {
                let task_name = columns[col_idx].to_string();

                let value = match sheet.cell(row_idx as u32, col_idx as u32) {
                    Some(cell) => ods_cell_to_string(cell),
                    None => continue,
                };

                if value.trim().is_empty() || value == "0" {
                    continue;
                }

                user_id_column.push(user_id.clone());
                task_name_column.push(task_name);
                timespent_column.push(value);
                month_column.push(month_num.clone());
            }
        }
    }

    df!(
        "user_id" => user_id_column,
        "task_name" => task_name_column,
        "timespent_h" => timespent_column,
        "month" => month_column
    ).map_err(|e| format!("Failed to create DataFrame: {e}"))
}

pub fn get_team_members_df() -> Result<DataFrame, String> {
    let wb = read_ods_file()?;

    let sheet = get_sheet(&wb, "Team_reported_time")?;

    // Data starts from row 3 (index 2)
    let row_start = 2;
    
    let mut user_id_column: Vec<String> = vec![];
    let mut user_name_column: Vec<String> = vec![];
    let mut user_last_name_column: Vec<String> = vec![];
    let mut position_column: Vec<String> = vec![];
    let mut comment_column: Vec<String> = vec![];
    
    // Read data rows until empty cell in column A
    let mut row = row_start;
    loop {
        // Get user_id from column A
        let user_id = match sheet.cell(row, 0) {
            Some(cell) => ods_cell_to_string(cell),
            None => break, // Stop if cell A is empty
        };
        // Read other columns
        let user_name = match sheet.cell(row, 1) {
            Some(cell) => ods_cell_to_string(cell),
            None => break, // Stop if cell A is empty
        };
        let user_last_name = match sheet.cell(row, 2) {
            Some(cell) => ods_cell_to_string(cell),
            None => break, // Stop if cell A is empty
        };
        let position = match sheet.cell(row, 3) {
            Some(cell) => ods_cell_to_string(cell),
            None => break, // Stop if cell A is empty
        };
        let comment = match sheet.cell(row, 4) {
            Some(cell) => ods_cell_to_string(cell),
            None => break, // Stop if cell A is empty
        };
        
        // Add values to vectors
        user_id_column.push(user_id);
        user_name_column.push(user_name);
        user_last_name_column.push(user_last_name);
        position_column.push(position);
        comment_column.push(comment);
        
        row += 1;
    }

    df!(
        "user_id" => user_id_column,
        "user_name" => user_name_column,
        "user_last_name" => user_last_name_column,
        "position" => position_column,
        "comment" => comment_column
    ).map_err(|e| format!("Failed to create DataFrame: {e}"))
}


pub fn get_team_cost_df() -> Result<DataFrame, String> {
    let wb = read_ods_file()?;

    let sheet = get_sheet(&wb, "Team_total_cost")?;

    // Data starts from row 3 (index 2)
    let row_start = 2;
    
    // For monthly cost data (long format)
    let mut cost_user_ids: Vec<String> = vec![];
    let mut cost_values: Vec<String> = vec![];
    let mut cost_months: Vec<String> = vec![];
    let mut cost_currencies: Vec<String> = vec![];
    
    // Read data rows until empty cell in column A
    let mut row = row_start;
    loop {
        // Get user_id from column A
        let user_id = match sheet.cell(row, 0) {
            Some(cell) => ods_cell_to_string(cell),
            None => break, // Stop if cell A is empty
        };
        
        // Process monthly data (columns F through AH, which are indices 5 through 33)
        for col_idx in 5..34 {
            let month = (col_idx - 5 + 1).to_string(); // Calculate month number (1-29)
            let cost = match sheet.cell(row, col_idx) {
                Some(cell) => ods_cell_to_string(cell),
                None => continue, // Stop if cell A is empty
            };

            if cost.trim().is_empty() || cost == "0" {
                continue;
            }
            
            // Add to long format data
            cost_user_ids.push(user_id.clone());
            cost_values.push(cost);
            cost_months.push(month);
            cost_currencies.push("PLN".to_string());
        }
        
        row += 1;
    }

    // Create the cost dataframe in long format
    df!(
        "user_id" => cost_user_ids,
        "cost" => cost_values,
        "month" => cost_months,
        "currency" => cost_currencies
    ).map_err(|e| format!("Failed to create DataFrame: {e}"))
}


