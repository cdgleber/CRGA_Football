use std::env;
use reqwest::header::HeaderMap;
use serde_json::Value;
use tokio::{ fs::File, io::AsyncWriteExt };
use anyhow::Result;
use chrono::Local;
use dotenv::dotenv;

use CRGA_Football::betters::get_betters_picks;
use CRGA_Football::standings::get_team_records;
use CRGA_Football::page::{ PAGE_TOP, PAGE_MIDDLE, PAGE_BOTTOM };

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let api_key = env::var("RapidAPI_Key").expect("RapidAPI_Key must be set");

    get_standings(&api_key).await?;
    get_all_results(&api_key).await?;

    let standings = get_team_records()?;

    let filename = format!("C:\\DEVELOPMENT\\CRGA_Football\\index.html");
    let mut file = File::create(filename).await?;

    let to_add = format!(
        "<div class=\"center\">Updated: {} </div><br />\n",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    );
    file.write_all(to_add.as_bytes()).await?;

    file.write_all(PAGE_TOP.as_bytes()).await?;

    let mut betters = get_betters_picks()?;
    betters.sort_by_key(|b| b.better_score(&standings));
    betters.reverse();
    for better in &betters {
        let to_add = format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            better.name,
            better.better_score(&standings),
            better.better_wins(&standings),
            better.better_div_losses(&standings)
        );

        file.write_all(to_add.as_bytes()).await?;
    }

    file.write_all(PAGE_MIDDLE.as_bytes()).await?;

    for t in standings.values() {
        let to_add = format!(
            "<tr><td><img src=\"{}\" /></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td>\n",
            t.image.clone().unwrap_or("".to_string()),
            t.name,
            t.team_score(),
            t.wins,
            t.div_losses
        );

        file.write_all(to_add.as_bytes()).await?;

        file.write_all("<td class=\"small\">".as_bytes()).await?;

        let team_name = t.name.clone();

        for b in &betters {
            if b.picks.iter().any(|p| p == &team_name) {
                let to_add = format!("{}<br />", b.name);
                file.write_all(to_add.as_bytes()).await?;
            }
        }

        file.write_all("</td></tr>\n".as_bytes()).await?;
    }

    file.write_all(PAGE_BOTTOM.as_bytes()).await?;

    Ok(())
}

#[allow(unused)]
async fn get_standings(api_key: &str) -> Result<()> {
    let url = format!(
        "https://flashlive-sports.p.rapidapi.com/v1/tournaments/standings?tournament_season_id=UgPH6H7i&standing_type=overall&locale=en_INT&tournament_stage_id=lWha05Vk"
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        "X-RapidAPI-Host",
        reqwest::header::HeaderValue::from_static("flashlive-sports.p.rapidapi.com")
    );
    headers.insert("X-RapidAPI-Key", reqwest::header::HeaderValue::from_str(api_key)?);

    let client = reqwest::Client::new();
    let res = client.get(url).headers(headers).send().await?;

    let body = res.text().await?;

    let filename = format!("C:\\DEVELOPMENT\\CRGA_Football\\src\\bin\\standings.json");
    let mut file = File::create(filename).await?;
    file.write_all(body.as_bytes()).await?;

    Ok(())
}

#[allow(unused)]
async fn get_results(page: u8, api_key: &str) -> Result<String> {
    let url =
        format!("https://flashlive-sports.p.rapidapi.com/v1/tournaments/results?locale=en_INT&tournament_stage_id=lWha05Vk&page={}", page);

    let mut headers = HeaderMap::new();
    headers.insert(
        "X-RapidAPI-Host",
        reqwest::header::HeaderValue::from_static("flashlive-sports.p.rapidapi.com")
    );

    headers.insert("X-RapidAPI-Key", reqwest::header::HeaderValue::from_str(api_key)?);

    let client = reqwest::Client::new();
    let res = client.get(url).headers(headers).send().await?;

    let body = res.text().await?;

    let filename = format!("C:\\DEVELOPMENT\\CRGA_Football\\src\\bin\\results_{}.json", page);
    let mut file = File::create(filename).await?;
    file.write_all(body.as_bytes()).await?;

    Ok(body)
}

async fn get_all_results(api_key: &str) -> Result<()> {
    let mut page = 1u8;
    let body = get_results(page, api_key).await?;

    let v: Value = serde_json::from_str(&body)?;
    let events = v["DATA"][0]["EVENTS"].as_array().unwrap();

    let mut event_length = events.len();

    while event_length > 29 {
        page += 1;
        let body = get_results(page, api_key).await?;
        let internal: Value = serde_json::from_str(&body)?;

        if let Some(new_length) = internal["DATA"][0]["EVENTS"].as_array() {
            event_length = new_length.len();
        }

        if page > 20u8 {
            break;
        }
    }

    Ok(())
}

// ID:5
// NAME:"AMERICAN_FOOTBALL"

// SPORT_ID:5
// COUNTRY_ID:200
// STAGE_ID:"dd6l0LmE"
// COUNTRY_NAME:"USA"
// LEAGUE_NAME:"NFL"
// TOURNAMENT_IMAGE:"https://www.flashscore.com/res/image/data/K6oepJiU-fet3845Q.png"

// NAME:"USA: NFL"
// HEADER:"USA: NFL;184;lWha05Vk"
// NAME_PART_1:"USA"
// NAME_PART_2:"NFL"
// TOURNAMENT_TEMPLATE_ID:"rJVAIaHo"
// COUNTRY_ID:200
// COUNTRY_NAME:"USA"
// TOURNAMENT_STAGE_ID:"lWha05Vk"
// TOURNAMENT_TYPE:"t"
// TOURNAMENT_ID:"UgPH6H7i"
// SOURCE_TYPE:0
// HAS_LIVE_TABLE:0
// STANDING_INFO:1
// TEMPLATE_ID:"200_rJVAIaHo"
// TOURNAMENT_STAGE_TYPE:2
// SHORT_NAME:"NFL"
// URL:"/american-football/usa/nfl/"
// TOURNAMENT_IMAGE:"https://www.flashscore.com/res/image/data/lOrR70BO-f5kXl3OQ.png"
// SORT:"10USA 003......0000000000001000NFL 003......000"
// STAGES_COUNT:0
// TSS:""
// ZKL:""
// ZKU:""
// TOURNAMENT_SEASON_ID:"UgPH6H7i"
// CATEGORY_NAME:"USA"

// current win loss overall
// https://flashlive-sports.p.rapidapi.com/v1/tournaments/standings?tournament_season_id=UgPH6H7i&standing_type=overall&locale=en_INT&tournament_stage_id=lWha05Vk

// previous results
// https://flashlive-sports.p.rapidapi.com/v1/tournaments/results?locale=en_INT&tournament_stage_id=lWha05Vk&page=1
