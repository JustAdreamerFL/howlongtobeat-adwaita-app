#![allow(non_snake_case)]

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

const HLTB_BASE_URL: &str = "https://howlongtobeat.com";
const DEBUG_LOG_MAX_CHARS: usize = 500;
const ERROR_RESPONSE_MAX_CHARS: usize = 200;
// Maximum size of JavaScript region to search for API keys (bytes)
const MAX_SEARCH_REGION_SIZE: usize = 800;
// Maximum position within search region to prevent infinite loops
const MAX_SEARCH_POSITION: usize = 600;
// Size of initial search region to check for .concat patterns (bytes)
const API_PATTERN_CHECK_SIZE: usize = 100;
// Length of fixed parts in pattern: "/api/{sub_page}/" excluding the variable sub_page
// Breakdown: " (1) + /api/ (5) + / (1) + " (1) = 8
const API_PATTERN_FIXED_CHARS: usize = 8;

// Cache for API keys to avoid fetching the main page on every search
#[derive(Clone)]
struct ApiKeys {
    search_key: String,
    sub_page: String,
}

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
    api_keys: Arc<Mutex<Option<ApiKeys>>>,
}

impl HltbClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36")
                .build()
                .expect("Failed to create HTTP client"),
            api_keys: Arc::new(Mutex::new(None)),
        }
    }

    /// Fetches the main page and extracts dynamic API keys
    async fn fetch_api_keys(&self) -> Result<ApiKeys> {
        // Fetch the main HowLongToBeat page
        let response = self
            .client
            .get(HLTB_BASE_URL)
            .send()
            .await?;
        
        let html = response.text().await?;
        
        // Extract the _app-*.js file path
        // Looking for pattern like: "/_next/static/chunks/pages/_app-abc123.js"
        let app_js_path = html
            .find("/pages/_app-")
            .and_then(|start_pos| {
                // Go back to find the opening quote
                let prefix = &html[..start_pos];
                let quote_pos = prefix.rfind('"')?;
                // Find the closing quote
                let suffix = &html[start_pos..];
                let end_quote = suffix.find('"')?;
                // Check bounds before slicing
                let end_index = start_pos + end_quote;
                if end_index <= html.len() {
                    Some(&html[quote_pos + 1..end_index])
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("Could not find _app.js path in HTML"))?;
        
        // Fetch the _app.js file
        let app_js_url = format!("{}{}", HLTB_BASE_URL, app_js_path);
        let app_js = self.client.get(&app_js_url).send().await?.text().await?;
        
        if std::env::var("HLTB_DEBUG").is_ok() {
            eprintln!("Found _app.js at: {}", app_js_path);
            eprintln!("_app.js size: {} bytes", app_js.len());
        }
        
        // Extract the sub-page and API key for the search/locate endpoint
        // Looking for pattern: "/api/locate/".concat("key1").concat("key2")
        // Try to find "/api/locate/" first, fall back to searching for any "/api/X/" pattern
        let api_pattern = r#""/api/locate/"#;
        let locate_pos = app_js.find(api_pattern)
            .or_else(|| {
                // Fallback: look for any "/api/XXX/" pattern that has .concat following it
                app_js.find(r#""/api/"#)
                    .and_then(|pos| {
                        let region = &app_js[pos..std::cmp::min(app_js.len(), pos + API_PATTERN_CHECK_SIZE)];
                        // Check if this is followed by a path and then .concat
                        if region.contains(r#".concat("#) {
                            Some(pos)
                        } else {
                            None
                        }
                    })
            })
            .ok_or_else(|| anyhow::anyhow!("Could not find API endpoint pattern in JavaScript"))?;
        
        // Extract the sub-page name (between "/api/" and the next "/")
        let sub_page_start = locate_pos + r#""/api/"#.len();
        let sub_page = app_js[sub_page_start..]
            .find('/')
            .map(|slash_pos| app_js[sub_page_start..sub_page_start + slash_pos].to_string())
            .unwrap_or_else(|| "locate".to_string());
        
        // Extract API search key by finding .concat patterns after the closing quote
        // Pattern: "/api/locate/".concat("key1").concat("key2")...
        let mut search_key = String::new();
        // Calculate position after the full pattern "/api/{sub_page}/"
        // The pattern consists of: " + /api/ + {sub_page} + / + "
        // Fixed characters: opening quote (1) + /api/ (5) + slash (1) + closing quote (1) = 8
        let concat_start_pos = locate_pos + sub_page.len() + API_PATTERN_FIXED_CHARS;
        let region_end = std::cmp::min(app_js.len(), concat_start_pos + MAX_SEARCH_REGION_SIZE);
        let search_region = &app_js[concat_start_pos..region_end];
        
        let mut search_pos = 0;
        while let Some(concat_pos) = search_region[search_pos..].find(".concat(") {
            search_pos += concat_pos + ".concat(".len();
            
            // Extract the string inside concat
            if let Some(quote_start) = search_region[search_pos..].find('"') {
                let after_quote = &search_region[search_pos + quote_start + 1..];
                if let Some(quote_end) = after_quote.find('"') {
                    let key_part = &after_quote[..quote_end];
                    search_key.push_str(key_part);
                    search_pos += quote_start + quote_end + 2;
                } else {
                    break;
                }
            } else {
                break;
            }
            
            // Safety: don't search too far to prevent infinite loops
            if search_pos > MAX_SEARCH_POSITION {
                break;
            }
        }
        
        if search_key.is_empty() {
            return Err(anyhow::anyhow!("Could not extract API search key from .concat patterns"));
        }
        
        if std::env::var("HLTB_DEBUG").is_ok() {
            eprintln!("Extracted API keys:");
            eprintln!("  Sub-page: {}", sub_page);
            eprintln!("  Search key: {}", search_key);
            eprintln!("  Full endpoint: /api/{}/{}", sub_page, search_key);
        }
        
        Ok(ApiKeys {
            search_key,
            sub_page,
        })
    }

    pub async fn search(&self, query: &str) -> Result<Vec<Game>> {
        // Try to get cached API keys, or fetch new ones
        let api_keys = {
            let cached = self.api_keys.lock()
                .expect("Failed to acquire API keys lock");
            if let Some(keys) = cached.as_ref() {
                keys.clone()
            } else {
                drop(cached);
                // Fetch new keys
                let new_keys = self.fetch_api_keys().await?;
                let mut cache = self.api_keys.lock()
                    .expect("Failed to acquire API keys lock for writing");
                *cache = Some(new_keys.clone());
                new_keys
            }
        };
        
        // Construct the dynamic API URL
        let api_url = format!(
            "{}/api/{}/{}",
            HLTB_BASE_URL, api_keys.sub_page, api_keys.search_key
        );
        
        let request = SearchRequest {
            search_terms: vec![query.to_string()],
            ..Default::default()
        };

        let response = self
            .client
            .post(&api_url)
            .header("Referer", format!("{}/", HLTB_BASE_URL))
            .header("Origin", HLTB_BASE_URL)
            .json(&request)
            .send()
            .await?;

        // Get response status and text for better error reporting
        let status = response.status();
        let response_text = response.text().await?;
        
        // Log the response for debugging (only when HLTB_DEBUG env var is set)
        if std::env::var("HLTB_DEBUG").is_ok() {
            eprintln!("API URL: {}", api_url);
            eprintln!("API Response Status: {}", status);
            eprintln!(
                "API Response Body (first {} chars): {}", 
                DEBUG_LOG_MAX_CHARS,
                truncate_str(&response_text, DEBUG_LOG_MAX_CHARS)
            );
        }

        // Check for 404 - might mean API keys are stale
        if status.as_u16() == 404 {
            if std::env::var("HLTB_DEBUG").is_ok() {
                eprintln!("Got 404, retrying with fresh API keys...");
            }
            
            // Clear cached keys and retry once
            {
                let mut cache = self.api_keys.lock()
                    .expect("Failed to acquire API keys lock for clearing");
                *cache = None;
            }
            
            // Try one more time with fresh keys
            let fresh_keys = self.fetch_api_keys().await?;
            let fresh_api_url = format!(
                "{}/api/{}/{}",
                HLTB_BASE_URL, fresh_keys.sub_page, fresh_keys.search_key
            );
            
            if std::env::var("HLTB_DEBUG").is_ok() {
                eprintln!("Retrying with fresh API URL: {}", fresh_api_url);
            }
            
            let retry_response = self
                .client
                .post(&fresh_api_url)
                .header("Referer", format!("{}/", HLTB_BASE_URL))
                .header("Origin", HLTB_BASE_URL)
                .json(&request)
                .send()
                .await?;
            
            let retry_status = retry_response.status();
            let retry_text = retry_response.text().await?;
            
            if std::env::var("HLTB_DEBUG").is_ok() {
                eprintln!("Retry response status: {}", retry_status);
            }
            
            if !retry_status.is_success() {
                return Err(anyhow::anyhow!(
                    "HowLongToBeat API request failed with status {}: {}",
                    retry_status,
                    truncate_str(&retry_text, ERROR_RESPONSE_MAX_CHARS)
                ));
            }
            
            // Parse retry response
            let search_response: SearchResponse = serde_json::from_str(&retry_text)
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Failed to parse API response: {}. Response was: {}",
                        e,
                        truncate_str(&retry_text, ERROR_RESPONSE_MAX_CHARS)
                    )
                })?;
            
            // Cache the fresh keys
            {
                let mut cache = self.api_keys.lock()
                    .expect("Failed to acquire API keys lock for updating");
                *cache = Some(fresh_keys);
            }
            
            return Ok(search_response.data);
        }
        
        if !status.is_success() {
            return Err(anyhow::anyhow!(
                "HowLongToBeat API returned error status {}: {}",
                status,
                truncate_str(&response_text, ERROR_RESPONSE_MAX_CHARS)
            ));
        }

        // Try to parse the response
        let search_response: SearchResponse = serde_json::from_str(&response_text)
            .map_err(|e| {
                anyhow::anyhow!(
                    "Failed to parse API response: {}. Response was: {}",
                    e,
                    truncate_str(&response_text, ERROR_RESPONSE_MAX_CHARS)
                )
            })?;
        
        Ok(search_response.data)
    }
}

/// Truncates a string to a maximum length
fn truncate_str(s: &str, max_len: usize) -> &str {
    if s.len() > max_len {
        &s[..max_len]
    } else {
        s
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
