# UI Design Overview

## Application Interface

### Window Layout

```
╔═══════════════════════════════════════════════════════════════╗
║  HowLongToBeat                                         ≡  ⊗   ║  ← Header Bar
║  [🔍 Search for a game...                              ]      ║  ← Search Entry
╠═══════════════════════════════════════════════════════════════╣
║                                                               ║
║  EMPTY STATE (when no search):                               ║
║  ┌───────────────────────────────────────────────────────┐  ║
║  │                                                         │  ║
║  │                    🔍                                   │  ║
║  │                                                         │  ║
║  │              Search for a game                         │  ║
║  │                                                         │  ║
║  │    Enter a game title to see completion times         │  ║
║  │                                                         │  ║
║  └───────────────────────────────────────────────────────┘  ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
```

### Search Results View

```
╔═══════════════════════════════════════════════════════════════╗
║  HowLongToBeat                                         ≡  ⊗   ║
║  [🔍 zelda                                             ]      ║
╠═══════════════════════════════════════════════════════════════╣
║  ┌─────────────────────────────────────────────────────────┐ ║
║  │ The Legend of Zelda: Breath of the Wild            ▼   │ ║
║  │ Nintendo Switch                                          │ ║
║  │                                                          │ ║
║  │   Main Story              50.5h          (12,345 ratings)│ ║
║  │   Main + Extras           103.5h         (8,901 ratings) │ ║
║  │   Completionist          189.0h          (2,567 ratings) │ ║
║  │   All Styles              98.5h          (23,813 ratings)│ ║
║  │                                                          │ ║
║  │   [View on HowLongToBeat]                               │ ║
║  │                                                          │ ║
║  ├─────────────────────────────────────────────────────────┤ ║
║  │ The Legend of Zelda: Tears of the Kingdom           ▼   │ ║
║  │ Nintendo Switch                                          │ ║
║  ├─────────────────────────────────────────────────────────┤ ║
║  │ The Legend of Zelda: Ocarina of Time                ▼   │ ║
║  │ Nintendo 64                                              │ ║
║  └─────────────────────────────────────────────────────────┘ ║
╚═══════════════════════════════════════════════════════════════╝
```

### Loading State

```
╔═══════════════════════════════════════════════════════════════╗
║  HowLongToBeat                                         ≡  ⊗   ║
║  [🔍 mario                                             ]      ║
╠═══════════════════════════════════════════════════════════════╣
║  ┌─────────────────────────────────────────────────────────┐ ║
║  │ Searching...                                     ⟳      │ ║
║  └─────────────────────────────────────────────────────────┘ ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
```

### Error State

```
╔═══════════════════════════════════════════════════════════════╗
║  HowLongToBeat                                         ≡  ⊗   ║
║  [🔍 minecraft                                         ]      ║
╠═══════════════════════════════════════════════════════════════╣
║  ┌─────────────────────────────────────────────────────────┐ ║
║  │ Error                                                    │ ║
║  │ Failed to search: network error                         │ ║
║  └─────────────────────────────────────────────────────────┘ ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
```

### No Results State

```
╔═══════════════════════════════════════════════════════════════╗
║  HowLongToBeat                                         ≡  ⊗   ║
║  [🔍 asdfghjkl                                         ]      ║
╠═══════════════════════════════════════════════════════════════╣
║  ┌─────────────────────────────────────────────────────────┐ ║
║  │ No results found                                         │ ║
║  │ No games found for 'asdfghjkl'                          │ ║
║  └─────────────────────────────────────────────────────────┘ ║
║                                                               ║
╚═══════════════════════════════════════════════════════════════╝
```

## Widget Hierarchy

```
AdwApplicationWindow
└── gtk::Box (vertical)
    ├── AdwHeaderBar
    │   └── gtk::SearchEntry (title widget)
    └── gtk::Stack
        ├── AdwStatusPage (empty state)
        │   ├── Icon: "system-search-symbolic"
        │   ├── Title: "Search for a game"
        │   └── Description: "Enter a game title to see completion times"
        └── gtk::ScrolledWindow (results state)
            └── gtk::ListBox
                └── AdwExpanderRow (per game)
                    ├── Title: Game name
                    ├── Subtitle: Platform
                    └── gtk::Box (expandable content)
                        ├── gtk::Box (per completion time)
                        │   ├── gtk::Label (category name)
                        │   ├── gtk::Label (time value)
                        │   └── gtk::Label (rating count)
                        └── gtk::LinkButton (to HowLongToBeat.com)
```

## Design Features

### Colors & Theming
- Uses system theme (light/dark mode support via LibAdwaita)
- Accent colors from user's desktop settings
- Dim labels for secondary information (rating counts)

### Typography
- Title: Game name (larger, bold)
- Subtitle: Platform information (smaller, secondary color)
- Body: Completion times and categories
- Caption: Rating counts (small, dim)

### Spacing
- Consistent 12px spacing between elements
- Proper margins (12px) inside expanded rows
- 6px spacing for related elements (buttons)

### Interaction
- Hover effects on expandable rows
- Smooth expansion animation
- Search updates as you type (debounced)
- Loading spinner animation
- Link button hover/active states

### Responsive Design
- Minimum width: 360px
- Maximum width: Grows with window
- Scrollable results list
- Works on various screen sizes

### Accessibility
- Keyboard navigation support
- Screen reader friendly labels
- High contrast support
- Focus indicators
- Proper semantic structure

## User Flow

1. **Start**: User sees empty state with search prompt
2. **Type**: User starts typing in search field
3. **Loading**: Spinner shows while searching
4. **Results**: List of games appears
5. **Expand**: User clicks game to see details
6. **View Details**: Completion times displayed
7. **Navigate**: User can click link to open website
8. **New Search**: User types new query, results update

## Keyboard Shortcuts (Future Enhancement)

- `Ctrl+F`: Focus search bar
- `Ctrl+Q`: Quit application
- `Escape`: Clear search
- `↑/↓`: Navigate results
- `Enter`: Expand/collapse selected item
- `Tab`: Cycle through expandable items

## Future UI Enhancements

- Display game cover images
- Add platform filter dropdown
- Sort options (popularity, release date, etc.)
- Favorites/bookmarks system
- Recently searched games
- Dark mode toggle (if not system-wide)
- Export results to CSV
- Share functionality
- Comparison view (compare multiple games)
- Statistics view (personal gaming stats)
