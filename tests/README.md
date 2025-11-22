# Tests

This directory contains integration tests for the HowLongToBeat application.

## Running Tests

To run all tests:
```bash
cargo test
```

To run specific test files:
```bash
cargo test --test api_deserialization_tests
```

## Test Coverage

### API Deserialization Tests (`api_deserialization_tests.rs`)

These tests verify that the API response parsing is robust and can handle various JSON response formats from the HowLongToBeat API:

- **Complete Data**: Tests deserialization with all fields present
- **Minimal Data**: Tests with only required fields, ensuring defaults work
- **Extra Fields**: Ensures unknown API fields don't break parsing
- **Empty Strings**: Verifies empty string handling
- **Array Responses**: Tests parsing arrays of game objects
- **Search Response Structure**: Tests the full API response wrapper
- **Empty Results**: Tests handling of searches with no results

### Unit Tests in `src/api.rs`

The API module also contains inline unit tests that verify:

- Game struct deserialization with complete, minimal, and null data
- Time calculation methods (hours conversion)
- URL generation methods
- Search request serialization
- Default implementations

## Why These Tests Matter

The error "error decoding response body" that was reported in issue #X occurred because:

1. The API response fields didn't match the expected struct definitions
2. Missing fields caused deserialization failures
3. The error messages didn't provide enough context

These tests ensure that:

1. The app can handle missing or null fields gracefully using `#[serde(default)]`
2. API changes that add new fields won't break the app
3. Better error messages help diagnose issues when deserialization fails

## Adding New Tests

When adding new API endpoints or response structures:

1. Add unit tests in the relevant module file (e.g., `src/api.rs`)
2. Add integration tests in this directory for complex scenarios
3. Test with real API responses when possible
4. Test edge cases like empty results, null fields, and error responses
