# LyRemember Backend (Rust)

Backend library for LyRemember lyrics memorization application.

## Features

- SQLite database management
- User authentication (bcrypt + JWT)
- Song CRUD operations
- Automatic translation (LibreTranslate API)
- Automatic phonetic generation (PyO3 + Python libs)
- Practice session tracking
- Genius API integration (optional)

## Dependencies

### Rust
See `Cargo.toml` for Rust dependencies. Build with:
```bash
cargo build
```

### Python
PyO3 requires Python 3.8+ and the following packages:
```bash
pip install -r requirements.txt
```

## Structure

```
src/
├── lib.rs              # Main library entry point
├── models/             # Data models
│   ├── mod.rs
│   ├── user.rs
│   ├── song.rs
│   └── session.rs
├── db/                 # Database layer
│   ├── mod.rs
│   └── sqlite.rs
├── services/           # Business logic
│   ├── mod.rs
│   ├── auth.rs
│   ├── phonetic.rs     # PyO3 bridge
│   ├── translation.rs
│   ├── songs.rs
│   └── practice.rs
└── error.rs            # Error types
```

## Usage

```rust
use lyremember_backend::db::init_database;
use lyremember_backend::services::phonetic::generate_phonetic;

// Initialize database
let conn = init_database("lyremember.db")?;

// Generate phonetics for Japanese text
let lyrics = vec!["千本桜".to_string(), "夜ニ紛レ".to_string()];
let romaji = generate_phonetic(lyrics, "jp")?;
```

## Testing

```bash
cargo test
```

## Integration with Tauri

This library is designed to be used as a dependency in a Tauri application:

```toml
# In Tauri's Cargo.toml
[dependencies]
lyremember_backend = { path = "../rust-backend" }
```

## License

MIT
