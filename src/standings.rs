use serde_json::Value;
use anyhow::Result;
use std::collections::BTreeMap;

pub fn get_team_records() -> Result<BTreeMap<String, Team>> {
    let event_json = std::fs::read_to_string("src/data/standings.json").unwrap();
    let v: Value = serde_json::from_str(&event_json)?;

    let afc = v["DATA"][1]["ROWS"].as_array().unwrap();

    let mut teams: BTreeMap<String, Team> = BTreeMap::new();
    for value in afc {
        let name = value["TEAM_NAME"].to_string().replace("\"", "");
        let wins = u8::try_from(value["WINS"].as_u64().unwrap()).unwrap();
        let matches = u8::try_from(value["MATCHES_PLAYED"].as_u64().unwrap()).unwrap();
        let losses = matches - wins;
        let div_losses = get_team_div_losses(&name).unwrap();
        let image = Some(value["TEAM_IMAGE_PATH"].to_string().replace("\"", ""));

        teams.insert(name.clone(), Team {
            name,
            wins,
            losses,
            div_losses,
            image,
        });
    }

    let nfc = v["DATA"][2]["ROWS"].as_array().unwrap();

    for value in nfc {
        let name = value["TEAM_NAME"].to_string().replace("\"", "");
        let wins = u8::try_from(value["WINS"].as_u64().unwrap()).unwrap();
        let matches = u8::try_from(value["MATCHES_PLAYED"].as_u64().unwrap()).unwrap();
        let losses = matches - wins;
        let div_losses = get_team_div_losses(&name).unwrap();
        let image = Some(value["TEAM_IMAGE_PATH"].to_string().replace("\"", ""));

        teams.insert(name.clone(), Team {
            name,
            wins,
            losses,
            div_losses,
            image,
        });
    }

    // println!("{:?}", teams.len());
    // println!("{:#?}", teams);

    Ok(teams)
}

#[allow(unused_assignments)]
pub fn get_team_div_losses(name: &str) -> Result<u8> {
    let mut page = 1;
    let mut div_losses = 0u8;
    loop {
        let path = format!("src/data/results_{}.json", page);

        let event_json = std::fs::read_to_string(path).unwrap();
        let v: Value = serde_json::from_str(&event_json)?;

        let events = v["DATA"][0]["EVENTS"].as_array().unwrap();
        let events_length = events.len();
        // println!("{}", events_length);

        for event in events {
            let mut loser = String::new();
            let mut winner = String::new();

            // println!("{:?}", event);

            match event["WINNER"].as_u64().unwrap() {
                1 => {
                    loser = event["AWAY_NAME"].to_string().replace("\"", "");
                    winner = event["HOME_NAME"].to_string().replace("\"", "");
                } //if winner = 1 then home won and away lost
                2 => {
                    loser = event["HOME_NAME"].to_string().replace("\"", "");
                    winner = event["AWAY_NAME"].to_string().replace("\"", "");
                } //if winner = 2 then away won and home lost
                _ => panic!("no winner."),
            }

            if name == &loser && return_division(&loser) == return_division(&winner) {
                div_losses += 1;

                // println!(
                //     "{name} {loser} {winner} {} {}",
                //     return_division(&loser),
                //     return_division(&winner)
                // );
            }
        }

        if events_length < 30 {
            break;
        }

        page += 1;
    }

    Ok(div_losses)
}

fn return_division(name: &str) -> String {
    match name {
        "New England Patriots" => "AFC East Division".to_string(),
        "Buffalo Bills" => "AFC East Division".to_string(),
        "Miami Dolphins" => "AFC East Division".to_string(),
        "New York Jets" => "AFC East Division".to_string(),
        "Pittsburgh Steelers" => "AFC North Division".to_string(),
        "Cincinnati Bengals" => "AFC North Division".to_string(),
        "Baltimore Ravens" => "AFC North Division".to_string(),
        "Cleveland Browns" => "AFC North Division".to_string(),
        "Houston Texans" => "AFC South Division".to_string(),
        "Indianapolis Colts" => "AFC South Division".to_string(),
        "Jacksonville Jaguars" => "AFC South Division".to_string(),
        "Tennessee Titans" => "AFC South Division".to_string(),
        "Los Angeles Chargers" => "AFC West Division".to_string(),
        "Kansas City Chiefs" => "AFC West Division".to_string(),
        "Denver Broncos" => "AFC West Division".to_string(),
        "Las Vegas Raiders" => "AFC West Division".to_string(),
        "Dallas Cowboys" => "NFC East Division".to_string(),
        "Philadelphia Eagles" => "NFC East Division".to_string(),
        "Washington Commanders" => "NFC East Division".to_string(),
        "New York Giants" => "NFC East Division".to_string(),
        "Minnesota Vikings" => "NFC North Division".to_string(),
        "Chicago Bears" => "NFC North Division".to_string(),
        "Detroit Lions" => "NFC North Division".to_string(),
        "Green Bay Packers" => "NFC North Division".to_string(),
        "New Orleans Saints" => "NFC South Division".to_string(),
        "Tampa Bay Buccaneers" => "NFC South Division".to_string(),
        "Atlanta Falcons" => "NFC South Division".to_string(),
        "Carolina Panthers" => "NFC South Division".to_string(),
        "San Francisco 49ers" => "NFC West Division".to_string(),
        "Seattle Seahawks" => "NFC West Division".to_string(),
        "Los Angeles Rams" => "NFC West Division".to_string(),
        "Arizona Cardinals" => "NFC West Division".to_string(),
        _ => panic!("didn't match division"),
    }
}

#[derive(Debug)]
pub struct Team {
    pub name: String,
    pub wins: u8,
    pub losses: u8,
    pub div_losses: u8,
    pub image: Option<String>,
}

impl Team {
    pub fn team_score(&self) -> i16 {
        let wins = i16::try_from(self.wins).unwrap();
        let div_losses = i16::try_from(self.div_losses).unwrap();
        wins.saturating_sub(div_losses)
    }
}
