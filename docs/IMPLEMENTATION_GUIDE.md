# LyRemember - Tauri + Vue Implementation Guide

## Quick Start

### Phase 1: Create Tauri Project

```bash
# Create new Tauri + Vue project
npm create tauri-app@latest

# Choose options:
# - Project name: lyremember-app
# - Package manager: npm
# - UI template: Vue
# - UI flavor: TypeScript
# - Add "@tauri-apps/api" npm package: Yes
# - Add "@tauri-apps/plugin-shell" npm package: No
```

### Phase 2: Navigate and Install

```bash
cd lyremember-app
npm install
```

### Phase 3: Add Tailwind CSS

```bash
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

Configure `tailwind.config.js`:
```javascript
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

Add to `src/style.css`:
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

### Phase 4: Add Shadcn-vue

```bash
npx shadcn-vue@latest init
# Follow prompts for setup

# Add components
npx shadcn-vue@latest add button
npx shadcn-vue@latest add card
npx shadcn-vue@latest add input
npx shadcn-vue@latest add dialog
npx shadcn-vue@latest add select
npx shadcn-vue@latest add slider
npx shadcn-vue@latest add badge
npx shadcn-vue@latest add tabs
```

### Phase 5: Add Additional Dependencies

```bash
# Vue ecosystem
npm install pinia vue-router vue-i18n@9

# Icons
npm install lucide-vue-next

# Utilities
npm install @vueuse/core
```

### Phase 6: Configure Rust Backend

Add to `src-tauri/Cargo.toml`:
```toml
[dependencies]
tauri = { version = "1.5", features = [] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rusqlite = { version = "0.30", features = ["bundled"] }
reqwest = { version = "0.11", features = ["json"] }
tokio = { version = "1", features = ["full"] }
bcrypt = "0.15"
jsonwebtoken = "9.2"
pyo3 = { version = "0.20", features = ["auto-initialize"] }
```

Create Python requirements:
```bash
# src-tauri/requirements.txt
pykakasi>=2.2.1
hangul-romanize>=0.1.0
epitran>=1.24
```

### Phase 7: Run Development Server

```bash
npm run tauri dev
```

### Phase 8: Build for Production

```bash
npm run tauri build
```

## Project Structure

```
lyremember-app/
├── src/                        # Vue frontend
│   ├── main.ts
│   ├── App.vue
│   ├── router/
│   │   └── index.ts
│   ├── stores/
│   │   ├── auth.ts
│   │   └── songs.ts
│   ├── views/
│   │   ├── LoginView.vue
│   │   ├── SongsView.vue
│   │   ├── SongDetailView.vue
│   │   └── PracticeView.vue
│   ├── components/
│   │   ├── ui/                 # Shadcn-vue components
│   │   ├── SongCard.vue
│   │   ├── PhoneticDisplay.vue
│   │   └── KaraokeMode.vue
│   └── lib/
│       └── tauri.ts            # API client
│
├── src-tauri/                  # Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── requirements.txt        # Python dependencies
│   └── src/
│       ├── main.rs
│       ├── models/
│       │   ├── mod.rs
│       │   ├── user.rs
│       │   └── song.rs
│       ├── db/
│       │   ├── mod.rs
│       │   └── sqlite.rs
│       ├── services/
│       │   ├── mod.rs
│       │   ├── phonetic.rs     # PyO3
│       │   ├── translation.rs
│       │   └── genius.rs
│       └── commands/
│           ├── mod.rs
│           ├── auth.rs
│           ├── songs.rs
│           └── practice.rs
│
├── package.json
├── vite.config.ts
├── tailwind.config.js
└── tsconfig.json
```

## Development Workflow

### 1. Start dev server
```bash
npm run tauri dev
```

### 2. Frontend development
- Edit Vue files in `src/`
- Hot reload automatic

### 3. Backend development
- Edit Rust files in `src-tauri/src/`
- Restart dev server to see changes

### 4. Database development
```bash
# SQLite database will be created at:
# ~/.lyremember/lyremember.db
```

## Commands Reference

### Frontend (Vue)
```bash
npm run dev          # Vite dev server only
npm run build        # Build frontend
npm run preview      # Preview build
```

### Tauri (Full Stack)
```bash
npm run tauri dev    # Dev with Tauri
npm run tauri build  # Production build
npm run tauri icon   # Generate app icons
```

## Next Steps

1. ✅ Setup project structure
2. ⏳ Implement Rust backend
3. ⏳ Implement Vue frontend
4. ⏳ Test features
5. ⏳ Build for production

## Notes

- Python must be installed on dev machine for PyO3
- For distribution, Python can be embedded in app
- SQLite database is local (offline-first)
- Translations and phonetics generated once, stored locally
