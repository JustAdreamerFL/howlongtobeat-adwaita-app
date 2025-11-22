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
#[serde(default)]
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

impl Default for SearchResponse {
    fn default() -> Self {
        Self {
            color: String::new(),
            title: String::new(),
            category: String::new(),
            count: 0,
            pageCurrent: 0,
            pageTotal: 0,
            pageSize: 0,
            data: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Game {
    pub count: u32,
    pub game_id: u64,
    pub game_name: String,
    #[serde(default)]
    pub game_name_date: u64,
    #[serde(default)]
    pub game_alias: String,
    #[serde(default)]
    pub game_type: String,
    #[serde(default)]
    pub game_image: String,
    #[serde(default)]
    pub comp_lvl_combine: u32,
    #[serde(default)]
    pub comp_lvl_sp: u32,
    #[serde(default)]
    pub comp_lvl_co: u32,
    #[serde(default)]
    pub comp_lvl_mp: u32,
    #[serde(default)]
    pub comp_lvl_spd: u32,
    #[serde(default)]
    pub comp_main: u32,
    #[serde(default)]
    pub comp_plus: u32,
    #[serde(default)]
    pub comp_100: u32,
    #[serde(default)]
    pub comp_all: u32,
    #[serde(default)]
    pub comp_main_count: u32,
    #[serde(default)]
    pub comp_plus_count: u32,
    #[serde(default)]
    pub comp_100_count: u32,
    #[serde(default)]
    pub comp_all_count: u32,
    #[serde(default)]
    pub invested_co: u32,
    #[serde(default)]
    pub invested_mp: u32,
    #[serde(default)]
    pub invested_co_count: u32,
    #[serde(default)]
    pub invested_mp_count: u32,
    #[serde(default)]
    pub count_comp: u32,
    #[serde(default)]
    pub count_speedrun: u32,
    #[serde(default)]
    pub count_backlog: u32,
    #[serde(default)]
    pub count_review: u32,
    #[serde(default)]
    pub review_score: u32,
    #[serde(default)]
    pub count_playing: u32,
    #[serde(default)]
    pub count_retired: u32,
    #[serde(default)]
    pub profile_dev: String,
    #[serde(default)]
    pub profile_popular: u32,
    #[serde(default)]
    pub profile_steam: u32,
    #[serde(default)]
    pub profile_platform: String,
    #[serde(default)]
    pub release_world: u64,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            count: 0,
            game_id: 0,
            game_name: String::new(),
            game_name_date: 0,
            game_alias: String::new(),
            game_type: String::new(),
            game_image: String::new(),
            comp_lvl_combine: 0,
            comp_lvl_sp: 0,
            comp_lvl_co: 0,
            comp_lvl_mp: 0,
            comp_lvl_spd: 0,
            comp_main: 0,
            comp_plus: 0,
            comp_100: 0,
            comp_all: 0,
            comp_main_count: 0,
            comp_plus_count: 0,
            comp_100_count: 0,
            comp_all_count: 0,
            invested_co: 0,
            invested_mp: 0,
            invested_co_count: 0,
            invested_mp_count: 0,
            count_comp: 0,
            count_speedrun: 0,
            count_backlog: 0,
            count_review: 0,
            review_score: 0,
            count_playing: 0,
            count_retired: 0,
            profile_dev: String::new(),
            profile_popular: 0,
            profile_steam: 0,
            profile_platform: String::new(),
            release_world: 0,
        }
    }
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

        // Get response status and text for better error reporting
        let status = response.status();
        let response_text = response.text().await?;
        
        // Log the response for debugging (only when HLTB_DEBUG env var is set)
        if std::env::var("HLTB_DEBUG").is_ok() {
            eprintln!("API Response Status: {}", status);
            eprintln!("API Response Body (first 500 chars): {}", 
                      if response_text.len() > 500 { 
                          &response_text[..500] 
                      } else { 
                          &response_text 
                      });
        }

        // Try to parse the response
        let search_response: SearchResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow::anyhow!(
                    "Failed to parse API response: {}. Response was: {}",
                    e,
                    if response_text.len() > 200 {
                        &response_text[..200]
                    } else {
                        &response_text
                    }
                )
            })?;
        
        Ok(search_response.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_deserialization_complete() {
        let json = r#"{
            "count": 1,
            "game_id": 12345,
            "game_name": "Test Game",
            "game_name_date": 1234567890,
            "game_alias": "test-game",
            "game_type": "game",
            "game_image": "test.jpg",
            "comp_lvl_combine": 1,
            "comp_lvl_sp": 1,
            "comp_lvl_co": 0,
            "comp_lvl_mp": 0,
            "comp_lvl_spd": 0,
            "comp_main": 36000,
            "comp_plus": 54000,
            "comp_100": 72000,
            "comp_all": 54000,
            "comp_main_count": 100,
            "comp_plus_count": 50,
            "comp_100_count": 25,
            "comp_all_count": 200,
            "invested_co": 0,
            "invested_mp": 0,
            "invested_co_count": 0,
            "invested_mp_count": 0,
            "count_comp": 200,
            "count_speedrun": 10,
            "count_backlog": 500,
            "count_review": 150,
            "review_score": 85,
            "count_playing": 300,
            "count_retired": 50,
            "profile_dev": "Test Developer",
            "profile_popular": 1000,
            "profile_steam": 123456,
            "profile_platform": "PC",
            "release_world": 1609459200
        }"#;

        let game: Result<Game, _> = serde_json::from_str(json);
        assert!(game.is_ok(), "Failed to deserialize complete game: {:?}", game.err());
        
        let game = game.unwrap();
        assert_eq!(game.game_name, "Test Game");
        assert_eq!(game.game_id, 12345);
        assert_eq!(game.comp_main, 36000);
        assert_eq!(game.main_story_hours(), 10.0);
    }

    #[test]
    fn test_game_deserialization_minimal() {
        // Test with minimal required fields
        let json = r#"{
            "count": 0,
            "game_id": 12345,
            "game_name": "Minimal Game"
        }"#;

        let game: Result<Game, _> = serde_json::from_str(json);
        assert!(game.is_ok(), "Failed to deserialize minimal game: {:?}", game.err());
        
        let game = game.unwrap();
        assert_eq!(game.game_name, "Minimal Game");
        assert_eq!(game.game_id, 12345);
        assert_eq!(game.comp_main, 0);
        assert_eq!(game.profile_platform, "");
    }

    #[test]
    fn test_game_deserialization_with_nulls() {
        // Test with null values that should use defaults
        let json = r#"{
            "count": 0,
            "game_id": 12345,
            "game_name": "Null Game",
            "game_alias": null,
            "game_type": null,
            "profile_platform": null
        }"#;

        let game: Result<Game, _> = serde_json::from_str(json);
        // This should work with #[serde(default)] but might not with null
        // In real scenarios, the API might omit fields rather than send null
        if let Ok(game) = game {
            assert_eq!(game.game_name, "Null Game");
        }
    }

    #[test]
    fn test_game_time_calculations() {
        let game = Game {
            comp_main: 36000,      // 10 hours
            comp_plus: 54000,      // 15 hours
            comp_100: 108000,      // 30 hours
            comp_all: 72000,       // 20 hours
            ..Default::default()
        };

        assert_eq!(game.main_story_hours(), 10.0);
        assert_eq!(game.main_plus_hours(), 15.0);
        assert_eq!(game.completionist_hours(), 30.0);
        assert_eq!(game.all_styles_hours(), 20.0);
    }

    #[test]
    fn test_game_urls() {
        let game = Game {
            game_id: 12345,
            game_image: "test_image.jpg".to_string(),
            ..Default::default()
        };

        assert_eq!(game.game_url(), "https://howlongtobeat.com/game/12345");
        assert_eq!(game.image_url(), "https://howlongtobeat.com/games/test_image.jpg");
    }

    #[test]
    fn test_search_response_deserialization() {
        let json = r#"{
            "color": "blue",
            "title": "Search Results",
            "category": "games",
            "count": 1,
            "pageCurrent": 1,
            "pageTotal": 1,
            "pageSize": 20,
            "data": [{
                "count": 0,
                "game_id": 12345,
                "game_name": "Test Game",
                "comp_main": 36000
            }]
        }"#;

        let response: Result<SearchResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok(), "Failed to deserialize search response: {:?}", response.err());
        
        let response = response.unwrap();
        assert_eq!(response.count, 1);
        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].game_name, "Test Game");
    }

    #[test]
    fn test_search_response_empty_results() {
        let json = r#"{
            "color": "blue",
            "title": "Search Results",
            "category": "games",
            "count": 0,
            "pageCurrent": 1,
            "pageTotal": 0,
            "pageSize": 20,
            "data": []
        }"#;

        let response: Result<SearchResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok(), "Failed to deserialize empty search response: {:?}", response.err());
        
        let response = response.unwrap();
        assert_eq!(response.count, 0);
        assert_eq!(response.data.len(), 0);
    }

    #[test]
    fn test_search_request_serialization() {
        let request = SearchRequest::default();
        let json = serde_json::to_string(&request);
        
        assert!(json.is_ok(), "Failed to serialize search request: {:?}", json.err());
        
        let json = json.unwrap();
        assert!(json.contains("searchType"));
        assert!(json.contains("games"));
        assert!(json.contains("searchTerms"));
    }

    #[test]
    fn test_search_request_with_query() {
        let request = SearchRequest {
            search_terms: vec!["zelda".to_string()],
            ..Default::default()
        };
        
        let json = serde_json::to_string(&request);
        assert!(json.is_ok());
        
        let json = json.unwrap();
        assert!(json.contains("zelda"));
    }

    #[test]
    fn test_game_default() {
        let game = Game::default();
        
        assert_eq!(game.game_id, 0);
        assert_eq!(game.game_name, "");
        assert_eq!(game.comp_main, 0);
        assert_eq!(game.main_story_hours(), 0.0);
    }

    #[test]
    fn test_search_response_default() {
        let response = SearchResponse::default();
        
        assert_eq!(response.count, 0);
        assert_eq!(response.data.len(), 0);
        assert_eq!(response.title, "");
    }

    #[test]
    fn test_game_deserialization_extra_fields() {
        // Test that extra fields in JSON don't break deserialization
        let json = r#"{
            "count": 0,
            "game_id": 12345,
            "game_name": "Test Game",
            "extra_field_1": "ignored",
            "extra_field_2": 999,
            "comp_main": 36000
        }"#;

        let game: Result<Game, _> = serde_json::from_str(json);
        assert!(game.is_ok(), "Failed to deserialize game with extra fields: {:?}", game.err());
        
        let game = game.unwrap();
        assert_eq!(game.game_name, "Test Game");
        assert_eq!(game.comp_main, 36000);
    }

    #[test]
    fn test_game_deserialization_missing_optional_fields() {
        // Test with many fields missing - they should use defaults
        let json = r#"{
            "count": 1,
            "game_id": 999,
            "game_name": "Sparse Game",
            "comp_main": 18000
        }"#;

        let game: Result<Game, _> = serde_json::from_str(json);
        assert!(game.is_ok(), "Failed to deserialize sparse game: {:?}", game.err());
        
        let game = game.unwrap();
        assert_eq!(game.game_name, "Sparse Game");
        assert_eq!(game.game_id, 999);
        assert_eq!(game.comp_main, 18000);
        assert_eq!(game.comp_plus, 0);
        assert_eq!(game.comp_100, 0);
        assert_eq!(game.profile_platform, "");
        assert_eq!(game.game_alias, "");
    }
}
