use std::fs;

fn temp_converter(input: Option<String>) -> Result<f64, String> {
    input
        .ok_or("Brak danych!".to_string())?
        .parse::<f64>()
        .map_err(|_| "zła liczba!".to_string())
}

fn temp_converter_fs(f_path: &str) -> Result<i32, String> {
    let content = fs::read_to_string(f_path).map_err(|_| "Error reading file!".to_string())?;
    let f_line: Option<String> = content.lines().next().map(String::from);
    let line = f_line.ok_or("Empty file!".to_string())?;
    line.trim().parse::<i32>().map_err(|_| "Parse Error".to_string())
}

fn main() {
    let tmp = Some("23.5".to_string());
    println!("{:?}", temp_converter(tmp)); 
}