# Fix Summary: HowLongToBeat API 404 Error

## Problem
The application was returning "404 Not Found" when trying to search for games via the HowLongToBeat API, even though it successfully extracted the API keys.

## Root Cause
After analyzing your working Python script, I found that the Rust implementation was missing a required field in the API request payload. The HowLongToBeat API expects a `lists` object in the `searchOptions`, but this was completely missing from the Rust code.

## Changes Made

### 1. Added Missing 'lists' Field
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

### 2. Fixed Search Terms Handling
Changed from sending the entire query as one string to splitting it into words:
```rust
// Before: search_terms: vec![query.to_string()]
// After: 
let search_terms: Vec<String> = query
    .trim()
    .split_whitespace()
    .map(|s| s.to_string())
    .collect();
```

This ensures searches like "minecraft java" are sent as `["minecraft", "java"]` instead of `["minecraft java"]`.

### 3. Enhanced Debugging
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
