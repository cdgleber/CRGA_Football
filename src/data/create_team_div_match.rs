use anyhow::Result;
use serde_json::Value;

// utility bin to create match for team and division
fn main() -> Result<()> {
    let event_json = std::fs
        ::read_to_string("C:\\DEVELOPMENT\\CRGA_Football\\src\\bin\\standings.json")
        .unwrap();
    let v: Value = serde_json::from_str(&event_json)?;

    let mut i = 3usize;

    while i < 11 {
        let division = v["DATA"][i]["DIVISION"].as_str().unwrap();
        let teams = v["DATA"][i]["ROWS"].as_array().unwrap();
        for v in teams {
            let team = v["TEAM_NAME"].as_str().unwrap();
            println!("\"{team}\" => \"{division}\".to_string(),");
        }
        i += 1;
    }

    Ok(())
}
