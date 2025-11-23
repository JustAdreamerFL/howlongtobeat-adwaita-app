# Troubleshooting Guide

This document provides solutions to common issues when using the HowLongToBeat application.

# Troubleshooting Guide

This document provides solutions to common issues when using the HowLongToBeat application.

## Search Errors

### "HowLongToBeat API endpoint not found (404)" or "Search Failed"

**Symptom**: When searching for a game, you see an error message with "404 Not Found".

**Recent Fix**: The API endpoint extraction logic has been improved to correctly identify the `/api/locate/` endpoint used for search. The application now automatically retries with fresh API keys when receiving a 404 error.

**Solution**: Make sure you're running the latest version:
```bash
git pull origin main
cargo build --release
```

If you're still experiencing issues:

1. **Enable debug logging** to see detailed information about API key extraction:
   ```bash
   HLTB_DEBUG=1 ./howlongtobeat
   # or
   HLTB_DEBUG=1 cargo run --release
   ```
   
   Debug output will show:
   - The _app.js file path and size
   - Extracted API sub-page and search key
   - Full API endpoint URL being used
   - Response status and body preview
   - Retry attempts with fresh keys

2. **Check the terminal output** for error messages. Look for:
   - `Found _app.js at: ...` - confirms JavaScript file was found
   - `Extracted API keys:` - shows the extracted endpoint details
   - `Got 404, retrying with fresh API keys...` - indicates retry attempt

3. **Verify network connectivity** to https://howlongtobeat.com/ in your browser

4. **Check for rate limiting** - if you're making many searches quickly, you might be temporarily blocked

5. **Wait and retry** - The API endpoint hashes may have changed during a website deployment. Wait a few minutes and try again.

6. **Report the issue** with debug output if the problem persists after trying the above steps

**Technical Note**: The HowLongToBeat website uses dynamic API endpoint URLs that include hash values (e.g., `/api/locate/45b48b2d1685d24b`). These hashes can change when the website is updated. The application automatically handles this by extracting the current hashes from the website's JavaScript and retrying with fresh keys when a 404 occurs.

### "Failed to search: error decoding response body"

**Symptom**: When searching for a game, you see an error message saying "Failed to search: error decoding response body".

**Cause**: This error occurred in earlier versions due to strict JSON deserialization that couldn't handle missing or null fields from the HowLongToBeat API.

**Solution**: Update to the latest version. The fix includes:
- Added `#[serde(default)]` attributes to handle missing fields gracefully
- Improved error messages with detailed API response logging
- Added comprehensive tests to prevent regression

If you're still experiencing this issue on the latest version:
1. Enable debug logging by setting the `HLTB_DEBUG` environment variable:
   ```bash
   HLTB_DEBUG=1 ./howlongtobeat
   # or
   HLTB_DEBUG=1 cargo run --release
   ```
2. Check the terminal output for detailed error messages showing the API response
3. Report the issue with the API response details
4. Check your internet connection
5. Verify that https://howlongtobeat.com/ is accessible

### Search returns no results

**Possible causes**:
- No games match your search query
- API is temporarily unavailable
- Network connectivity issues

**Solutions**:
1. Try a different search term
2. Check if https://howlongtobeat.com/ is accessible in your browser
3. Run the app from terminal to see error messages: `./howlongtobeat`
4. Enable debug mode for more details: `HLTB_DEBUG=1 ./howlongtobeat`

## Build Errors

### Missing GTK/LibAdwaita dependencies

**Error**: `pkg-config` can't find `glib-2.0`, `gtk4`, or `libadwaita`

**Solution**: Install the required dependencies for your distribution:

**Ubuntu/Debian**:
```bash
sudo apt install libgtk-4-dev libadwaita-1-dev pkg-config
```

**Fedora**:
```bash
sudo dnf install gtk4-devel libadwaita-devel
```

**Arch Linux**:
```bash
sudo pacman -S gtk4 libadwaita
```

### Rust version too old

**Error**: Build fails with Rust syntax errors

**Solution**: Update Rust to version 1.70 or later:
```bash
rustup update
```

## Runtime Errors

### App crashes on startup

**Possible causes**:
- Missing runtime libraries
- Incompatible GTK/LibAdwaita versions

**Solutions**:
1. Ensure GTK4 >= 4.10 and LibAdwaita >= 1.5 are installed
2. Check terminal output for specific error messages
3. Try rebuilding: `cargo clean && cargo build --release`

### Network timeout errors

**Symptom**: Searches take a long time and eventually timeout

**Solutions**:
1. Check your internet connection speed
2. Verify you can access https://howlongtobeat.com/ in a browser
3. Check if you're behind a proxy or firewall that might be blocking the requests
4. The app uses standard HTTPS requests, so proxy settings from your environment should be respected

## Development/Testing Issues

### Tests fail to run

**Error**: `cargo test` fails with missing dependencies

**Solution**: Ensure all development dependencies are installed (same as build dependencies above)

### Integration tests fail

If integration tests in `tests/` directory fail:
1. Check that you have internet connectivity (some tests may require it in the future)
2. Ensure serde_json is properly installed: `cargo update`
3. Run specific test files: `cargo test --test api_deserialization_tests`

## Getting Help

If you encounter an issue not listed here:

1. **Check existing issues**: Look at the GitHub issues page to see if others have reported the same problem
2. **Enable debug output**: Run the app from terminal to see detailed error messages
3. **Report the issue**: Create a new GitHub issue with:
   - The error message (from terminal output)
   - Your OS and version
   - GTK4 and LibAdwaita versions
   - Steps to reproduce the error
   - Any relevant terminal output

## Recent Fixes

### Version 0.1.0
- Fixed "error decoding response body" by making API deserialization more flexible
- Added comprehensive tests for API response handling
- Improved error messages with detailed response logging
- Added support for handling missing/null fields in API responses
