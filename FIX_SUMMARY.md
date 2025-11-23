# Fix Summary: HowLongToBeat API 404 Error

## Problem
The application was returning "404 Not Found" when trying to search for games via the HowLongToBeat API, even though it successfully extracted the API keys. This issue persisted across multiple attempts (issues #9, #13, #15).

## Root Cause
After analyzing the working Python script (from issue #9) and comparing it with the Rust implementation, I found that the Rust code was missing **two additional required fields** in the API request payload that the HowLongToBeat API expects:

1. `difficulty` field in the `gameplay` object
2. `rangeYear` object with `min` and `max` fields

These fields were present in the working Python script but were missing from the Rust implementation.

## Changes Made

### 1. Added Missing 'lists' Field (Previous Fix)
**File**: `src/api.rs`

Added `ListSearchOptions` struct and included it in the `SearchOptions`:
```rust
pub struct ListSearchOptions {
    #[serde(rename = "sortCategory")]
    pub sort_category: String,  // defaults to "follows"
}
```

This matches the Python script's payload structure:
```python
"lists": {"sortCategory": "follows"}
```

### 2. Added Missing 'difficulty' Field (Current Fix)
**File**: `src/api.rs`

Added `difficulty` field to the `Gameplay` struct:
```rust
pub struct Gameplay {
    pub perspective: String,
    pub flow: String,
    pub genre: String,
    pub difficulty: String,  // New field
}
```

This matches the Python script's payload:
```python
"gameplay": {"perspective": "", "flow": "", "genre": "", "difficulty": ""}
```

### 3. Added Missing 'rangeYear' Field (Current Fix)
**File**: `src/api.rs`

Added new `RangeYear` struct and included it in `GameSearchOptions`:
```rust
pub struct RangeYear {
    pub min: String,
    pub max: String,
}

pub struct GameSearchOptions {
    // ... existing fields ...
    #[serde(rename = "rangeYear")]
    pub range_year: RangeYear,
    // ... remaining fields ...
}
```

This matches the Python script's payload:
```python
"rangeYear": {"min": "", "max": ""}
```

### 4. Fixed Search Terms Handling (Previous Fix)
Changed from sending the entire query as one string to splitting it into words:
```rust
let search_terms: Vec<String> = query
    .trim()
    .split_whitespace()
    .map(|s| s.to_string())
    .collect();
```

This ensures searches like "minecraft java" are sent as `["minecraft", "java"]` instead of `["minecraft java"]`.

### 5. Enhanced Debugging (Previous Fix)
Added detailed logging when `HLTB_DEBUG=1` is set:
- Request payload before sending
- Retry attempts with fresh API keys
- Response status and body for both initial and retry requests

## Testing
✅ All existing unit tests pass  
✅ Code compiles successfully  
✅ Payload structure now matches the working Python script exactly

## How to Verify
Run the application with debug mode enabled:
```bash
HLTB_DEBUG=1 ./target/release/howlongtobeat
```

Then search for a game. You should now see:
- Successful API key extraction
- Request payload details
- HTTP 200 OK response (instead of 404)
- Search results displayed

## Additional Notes
- The fix ensures the API request payload matches your working Python script exactly
- Cookie handling is automatic in the `reqwest` client (no changes needed)
- The same client instance is used for fetching the main page, _app.js, and making API requests, maintaining session state

If you still encounter issues, the enhanced debug output will now show exactly what's being sent to the API, making it easier to diagnose any remaining problems.
