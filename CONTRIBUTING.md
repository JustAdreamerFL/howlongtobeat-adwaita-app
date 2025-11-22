# Contributing to HowLongToBeat

Thank you for your interest in contributing to HowLongToBeat! This document provides guidelines for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/howlongtobeat-adwaita-app.git
   cd howlongtobeat-adwaita-app
   ```
3. **Install dependencies** (see INSTALL.md)
4. **Build the project**:
   ```bash
   cargo build
   ```
5. **Run the application**:
   ```bash
   cargo run
   ```

## Development Workflow

1. **Create a branch** for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following the code style guidelines below

3. **Test your changes**:
   ```bash
   cargo build --release
   cargo run --release
   ```

4. **Commit your changes**:
   ```bash
   git add .
   git commit -m "Description of your changes"
   ```

5. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```

6. **Create a Pull Request** on GitHub

## Code Style Guidelines

### Rust Code
- Follow standard Rust formatting (use `cargo fmt`)
- Run `cargo clippy` to catch common mistakes
- Write clear, self-documenting code
- Add comments for complex logic
- Keep functions focused and small

### Formatting
```bash
# Format your code
cargo fmt

# Check for common mistakes
cargo clippy

# Run both
cargo fmt && cargo clippy
```

### UI Guidelines
- Follow GNOME Human Interface Guidelines
- Use LibAdwaita widgets when possible
- Ensure responsive design
- Test on different screen sizes
- Maintain consistent spacing and alignment

## Areas for Contribution

### Features
- [ ] Caching search results
- [ ] Favorites/bookmarks system
- [ ] Platform filtering
- [ ] Display game cover images
- [ ] Dark mode support
- [ ] Keyboard shortcuts
- [ ] Export functionality
- [ ] Internationalization (i18n)

### Improvements
- [ ] Better error handling
- [ ] Loading state animations
- [ ] Search suggestions
- [ ] Accessibility improvements
- [ ] Performance optimizations
- [ ] Unit tests
- [ ] Integration tests

### Documentation
- [ ] Code documentation
- [ ] User guide
- [ ] Video tutorials
- [ ] Screenshots
- [ ] Translation guides

### Packaging
- [ ] Flatpak manifest
- [ ] AUR package
- [ ] Fedora COPR
- [ ] Ubuntu PPA

## Testing

Before submitting a pull request:

1. **Build the project**:
   ```bash
   cargo build --release
   ```

2. **Test the application**:
   - Search for various games
   - Test with network issues
   - Test UI responsiveness
   - Check for memory leaks

3. **Run clippy**:
   ```bash
   cargo clippy -- -D warnings
   ```

4. **Format code**:
   ```bash
   cargo fmt --check
   ```

## Commit Message Guidelines

- Use clear, descriptive commit messages
- Start with a verb (Add, Fix, Update, etc.)
- Keep the first line under 50 characters
- Add detailed description if needed

Examples:
```
Add favorites system

- Implement favorites storage
- Add UI for managing favorites
- Update documentation
```

```
Fix search not working with special characters

Properly encode special characters in search queries.
```

## Pull Request Guidelines

1. **Title**: Clear description of what the PR does
2. **Description**: Explain:
   - What changes were made
   - Why they were made
   - How to test them
3. **Testing**: Describe how you tested the changes
4. **Screenshots**: Add screenshots for UI changes
5. **Breaking Changes**: Clearly note any breaking changes

## Code Review Process

1. Maintainers will review your PR
2. Address any feedback or requested changes
3. Once approved, your PR will be merged

## Questions?

- Open an issue for questions
- Discuss features in issues before implementing
- Join community discussions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Thank You!

Your contributions help make HowLongToBeat better for everyone!
