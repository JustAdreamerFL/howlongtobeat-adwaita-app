#![allow(non_snake_case)]

use anyhow::Result;
use serde::{Deserialize, Serialize};

const HLTB_API_URL: &str = "https://howlongtobeat.com/api/search";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    #[serde(rename = "searchType")]
    pub search_type: String,
    #[serde(rename = "searchTerms")]
    pub search_terms: Vec<String>,
    #[serde(rename = "searchPage")]
    pub search_page: u32,
    pub size: u32,
    #[serde(rename = "searchOptions")]
    pub search_options: SearchOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SearchOptions {
    pub games: GameSearchOptions,
    pub users: UserSearchOptions,
    pub filter: String,
    pub sort: u32,
    pub randomizer: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSearchOptions {
    pub userId: u32,
    pub platform: String,
    #[serde(rename = "sortCategory")]
    pub sort_category: String,
    #[serde(rename = "rangeCategory")]
    pub range_category: String,
    #[serde(rename = "rangeTime")]
    pub range_time: RangeTime,
    pub gameplay: Gameplay,
    pub modifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSearchOptions {
    #[serde(rename = "sortCategory")]
    pub sort_category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct RangeTime {
    pub min: Option<u32>,
    pub max: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct Gameplay {
    pub perspective: String,
    pub flow: String,
    pub genre: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub color: String,
    pub title: String,
    pub category: String,
    pub count: u32,
    pub pageCurrent: u32,
    pub pageTotal: u32,
    pub pageSize: u32,
    pub data: Vec<Game>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub count: u32,
    pub game_id: u64,
    pub game_name: String,
    pub game_name_date: u64,
    pub game_alias: String,
    pub game_type: String,
    pub game_image: String,
    pub comp_lvl_combine: u32,
    pub comp_lvl_sp: u32,
    pub comp_lvl_co: u32,
    pub comp_lvl_mp: u32,
    pub comp_lvl_spd: u32,
    pub comp_main: u32,
    pub comp_plus: u32,
    pub comp_100: u32,
    pub comp_all: u32,
    pub comp_main_count: u32,
    pub comp_plus_count: u32,
    pub comp_100_count: u32,
    pub comp_all_count: u32,
    pub invested_co: u32,
    pub invested_mp: u32,
    pub invested_co_count: u32,
    pub invested_mp_count: u32,
    pub count_comp: u32,
    pub count_speedrun: u32,
    pub count_backlog: u32,
    pub count_review: u32,
    pub review_score: u32,
    pub count_playing: u32,
    pub count_retired: u32,
    pub profile_dev: String,
    pub profile_popular: u32,
    pub profile_steam: u32,
    pub profile_platform: String,
    pub release_world: u64,
}

impl Game {
    /// Get the main story completion time in hours
    pub fn main_story_hours(&self) -> f64 {
        self.comp_main as f64 / 3600.0
    }

    /// Get the main + extras completion time in hours
    pub fn main_plus_hours(&self) -> f64 {
        self.comp_plus as f64 / 3600.0
    }

    /// Get the completionist time in hours
    pub fn completionist_hours(&self) -> f64 {
        self.comp_100 as f64 / 3600.0
    }

    /// Get all styles completion time in hours
    pub fn all_styles_hours(&self) -> f64 {
        self.comp_all as f64 / 3600.0
    }

    /// Get the image URL
    #[allow(dead_code)]
    pub fn image_url(&self) -> String {
        format!("https://howlongtobeat.com/games/{}", self.game_image)
    }

    /// Get the game page URL
    pub fn game_url(&self) -> String {
        format!("https://howlongtobeat.com/game/{}", self.game_id)
    }
}

impl Default for SearchRequest {
    fn default() -> Self {
        Self {
            search_type: "games".to_string(),
            search_terms: Vec::new(),
            search_page: 1,
            size: 20,
            search_options: SearchOptions::default(),
        }
    }
}

impl Default for GameSearchOptions {
    fn default() -> Self {
        Self {
            userId: 0,
            platform: String::new(),
            sort_category: "popular".to_string(),
            range_category: "main".to_string(),
            range_time: RangeTime::default(),
            gameplay: Gameplay::default(),
            modifier: String::new(),
        }
    }
}

impl Default for UserSearchOptions {
    fn default() -> Self {
        Self {
            sort_category: "postcount".to_string(),
        }
    }
}

pub struct HltbClient {
    client: reqwest::Client,
}

impl HltbClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
                .build()
                .expect("Failed to create HTTP client"),
        }
    }

    pub async fn search(&self, query: &str) -> Result<Vec<Game>> {
        let request = SearchRequest {
            search_terms: vec![query.to_string()],
            ..Default::default()
        };

        let response = self
            .client
            .post(HLTB_API_URL)
            .header("Referer", "https://howlongtobeat.com/")
            .json(&request)
            .send()
            .await?;

        let search_response: SearchResponse = response.json().await?;
        Ok(search_response.data)
    }
}
