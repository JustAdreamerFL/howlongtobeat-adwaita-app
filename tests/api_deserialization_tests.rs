// Integration tests for API deserialization
// These tests verify that the API response structures can handle various JSON formats
// Run with: cargo test --test api_deserialization_tests

use serde_json;

// Minimal test struct that mirrors the Game structure
#[derive(Debug, serde::Deserialize)]
#[serde(default)]
struct TestGame {
    count: u32,
    game_id: u64,
    game_name: String,
    game_alias: String,
    comp_main: u32,
    profile_platform: String,
}

impl Default for TestGame {
    fn default() -> Self {
        Self {
            count: 0,
            game_id: 0,
            game_name: String::new(),
            game_alias: String::new(),
            comp_main: 0,
            profile_platform: String::new(),
        }
    }
}

// Test struct for search response
#[derive(Debug, serde::Deserialize)]
#[serde(default)]
struct TestSearchResponse {
    color: String,
    title: String,
    count: u32,
    data: Vec<TestGame>,
}

impl Default for TestSearchResponse {
    fn default() -> Self {
        Self {
            color: String::new(),
            title: String::new(),
            count: 0,
            data: Vec::new(),
        }
    }
}

#[test]
fn test_api_response_with_complete_data() {
    let json = r#"{
        "count": 1,
        "game_id": 12345,
        "game_name": "The Legend of Zelda",
        "game_alias": "zelda",
        "comp_main": 36000,
        "profile_platform": "Nintendo Switch"
    }"#;

    let result: Result<TestGame, _> = serde_json::from_str(json);
    assert!(
        result.is_ok(),
        "Should deserialize complete game data successfully"
    );

    let game = result.unwrap();
    assert_eq!(game.game_name, "The Legend of Zelda");
    assert_eq!(game.game_id, 12345);
    assert_eq!(game.comp_main, 36000);
}

#[test]
fn test_api_response_with_minimal_data() {
    // Test with only required fields - other fields should use defaults
    let json = r#"{
        "count": 0,
        "game_id": 999,
        "game_name": "Minimal Game"
    }"#;

    let result: Result<TestGame, _> = serde_json::from_str(json);
    assert!(
        result.is_ok(),
        "Should deserialize minimal game data with defaults: {:?}",
        result.err()
    );

    let game = result.unwrap();
    assert_eq!(game.game_name, "Minimal Game");
    assert_eq!(game.game_id, 999);
    assert_eq!(game.comp_main, 0, "Should use default value for missing field");
    assert_eq!(
        game.profile_platform, "",
        "Should use default value for missing field"
    );
}

#[test]
fn test_api_response_with_extra_fields() {
    // API might return additional fields we don't care about
    let json = r#"{
        "count": 1,
        "game_id": 555,
        "game_name": "Extra Fields Game",
        "unknown_field": "should be ignored",
        "another_unknown": 999,
        "comp_main": 72000
    }"#;

    let result: Result<TestGame, _> = serde_json::from_str(json);
    assert!(
        result.is_ok(),
        "Should deserialize despite extra unknown fields: {:?}",
        result.err()
    );

    let game = result.unwrap();
    assert_eq!(game.game_name, "Extra Fields Game");
    assert_eq!(game.comp_main, 72000);
}

#[test]
fn test_api_response_empty_strings() {
    // Test that empty strings are handled correctly
    let json = r#"{
        "count": 0,
        "game_id": 123,
        "game_name": "",
        "game_alias": "",
        "profile_platform": ""
    }"#;

    let result: Result<TestGame, _> = serde_json::from_str(json);
    assert!(result.is_ok(), "Should handle empty strings: {:?}", result.err());

    let game = result.unwrap();
    assert_eq!(game.game_name, "");
    assert_eq!(game.game_alias, "");
}

#[test]
fn test_api_response_array() {
    // Test deserializing an array of games
    let json = r#"[
        {
            "count": 1,
            "game_id": 1,
            "game_name": "Game 1",
            "comp_main": 10000
        },
        {
            "count": 2,
            "game_id": 2,
            "game_name": "Game 2"
        }
    ]"#;

    let result: Result<Vec<TestGame>, _> = serde_json::from_str(json);
    assert!(
        result.is_ok(),
        "Should deserialize array of games: {:?}",
        result.err()
    );

    let games = result.unwrap();
    assert_eq!(games.len(), 2);
    assert_eq!(games[0].game_name, "Game 1");
    assert_eq!(games[1].game_name, "Game 2");
    assert_eq!(games[1].comp_main, 0, "Should use default for missing field");
}

#[test]
fn test_search_response_structure() {
    let json = r#"{
        "color": "blue",
        "title": "Search Results",
        "count": 1,
        "data": [{
            "count": 0,
            "game_id": 777,
            "game_name": "Search Result Game",
            "comp_main": 50000
        }]
    }"#;

    let result: Result<TestSearchResponse, _> = serde_json::from_str(json);
    assert!(
        result.is_ok(),
        "Should deserialize search response: {:?}",
        result.err()
    );

    let response = result.unwrap();
    assert_eq!(response.count, 1);
    assert_eq!(response.data.len(), 1);
    assert_eq!(response.data[0].game_name, "Search Result Game");
}

#[test]
fn test_search_response_empty_results() {
    let json = r#"{
        "count": 0,
        "data": []
    }"#;

    let result: Result<TestSearchResponse, _> = serde_json::from_str(json);
    assert!(result.is_ok(), "Should handle empty results: {:?}", result.err());

    let response = result.unwrap();
    assert_eq!(response.count, 0);
    assert_eq!(response.data.len(), 0);
}
