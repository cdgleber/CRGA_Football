use std::collections::BTreeMap;
use anyhow::Result;

use crate::standings::Team;

pub fn get_betters_picks() -> Result<Vec<Better>> {
    let better_picks = include_str!("better_picks.txt");

    let betters: Vec<Better> = better_picks
        .split("\r\n\r\n")
        .map(|better| {
            let mut better_lines = better.lines();

            let name = better_lines.next().unwrap().to_string();
            let picks = better_lines.map(|t| { t.trim().to_string() }).collect::<Vec<String>>();

            Better {
                name,
                picks,
            }
        })
        .collect::<Vec<_>>();

    Ok(betters)
}

#[derive(Debug)]
pub struct Better {
    pub name: String,
    pub picks: Vec<String>,
}

impl Better {
    pub fn better_score(&self, standings: &BTreeMap<String, Team>) -> i16 {
        self.picks
            .iter()
            .map(|pick| { standings.get(pick).unwrap().team_score() })
            .sum()
    }

    pub fn better_wins(&self, standings: &BTreeMap<String, Team>) -> u8 {
        self.picks
            .iter()
            .map(|pick| { standings.get(pick).unwrap().wins })
            .sum()
    }

    pub fn better_div_losses(&self, standings: &BTreeMap<String, Team>) -> u8 {
        self.picks
            .iter()
            .map(|pick| { standings.get(pick).unwrap().div_losses })
            .sum()
    }
}
