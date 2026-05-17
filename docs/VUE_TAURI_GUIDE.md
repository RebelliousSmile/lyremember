# Tauri + Vue.js - Guide Complet

## ✅ Vue.js fonctionne PARFAITEMENT avec Tauri !

Tauri est **framework-agnostic** : vous pouvez utiliser n'importe quel framework frontend.

---

## 🎯 Stack Recommandée : Vue + Tauri

### Architecture

```
Frontend : Vue 3 + TypeScript + Vite
UI Library : Shadcn-vue (équivalent Vue de Shadcn)
Styling : Tailwind CSS
Icons : Lucide Vue (ou Heroicons)
State : Pinia (state management officiel Vue 3)
Router : Vue Router
Desktop : Tauri (Rust backend)
Mobile : Tauri Mobile (beta) ou PWA
Backend : Rust (Tauri commands)
BDD : SQLite (rusqlite)
```

---

## 🚀 Setup Tauri + Vue

### Méthode 1 : Create Tauri App (Recommandé)

```bash
# Créer nouveau projet
npm create tauri-app

# Choisir dans l'assistant :
# ✓ Project name: lyremember
# ✓ Choose your package manager: npm
# ✓ Choose your UI template: Vue
# ✓ Choose your UI flavor: TypeScript
```

### Méthode 2 : Ajouter Tauri à projet Vue existant

```bash
# Si vous avez déjà un projet Vue
npm create vue@latest

# Puis ajouter Tauri
npm install --save-dev @tauri-apps/cli
npx tauri init
```

---

## 🎨 Shadcn-vue (UI Components pour Vue)

**Shadcn-vue** = Port officiel de Shadcn/ui pour Vue 3 !

### Installation

```bash
# 1. Setup Vue project avec Tauri
npm create tauri-app

# 2. Ajouter Tailwind CSS
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p

# 3. Configurer Tailwind
# tailwind.config.js
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

# 4. Ajouter Shadcn-vue
npx shadcn-vue@latest init

# 5. Ajouter composants
npx shadcn-vue@latest add button
npx shadcn-vue@latest add card
npx shadcn-vue@latest add input
npx shadcn-vue@latest add dialog
npx shadcn-vue@latest add select
```

### Composants Disponibles

Plus de 45 composants :
- Accordion, Alert, Avatar, Badge, Button
- Card, Checkbox, Combobox, Command, Dialog
- Dropdown Menu, Form, Input, Label, Popover
- Select, Sheet, Slider, Switch, Table
- Tabs, Toast, Tooltip, etc.

---

## 📁 Structure Projet Vue + Tauri

```
lyremember/
├── src/                        # Frontend Vue
│   ├── main.ts                # Point d'entrée Vue
│   ├── App.vue                # Composant root
│   ├── router/
│   │   └── index.ts           # Vue Router
│   ├── stores/                # Pinia stores
│   │   ├── auth.ts
│   │   └── songs.ts
│   ├── views/                 # Pages
│   │   ├── LoginView.vue
│   │   ├── RegisterView.vue
│   │   ├── DashboardView.vue
│   │   ├── SongsView.vue
│   │   ├── SongDetailView.vue
│   │   ├── PracticeView.vue
│   │   └── StatsView.vue
│   ├── components/            # Composants
│   │   ├── ui/                # Shadcn-vue components
│   │   │   ├── button/
│   │   │   ├── card/
│   │   │   └── ...
│   │   ├── SongCard.vue
│   │   ├── PhoneticDisplay.vue
│   │   ├── KaraokeMode.vue
│   │   ├── FillBlankMode.vue
│   │   └── VoiceMode.vue
│   ├── lib/
│   │   └── tauri.ts           # API Tauri
│   ├── composables/           # Vue composables
│   │   └── useSongs.ts
│   ├── types/
│   │   └── index.ts
│   └── assets/
│
├── src-tauri/                 # Backend Rust
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── src/
│   │   ├── main.rs
│   │   ├── commands/
│   │   ├── models/
│   │   ├── db/
│   │   └── services/
│   └── icons/
│
├── package.json
├── vite.config.ts
├── tailwind.config.js
└── tsconfig.json
```

---

## 💻 Exemples de Code Vue

### 1. Composant SongCard

```vue
<!-- components/SongCard.vue -->
<script setup lang="ts">
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Badge } from '@/components/ui/badge'
import { Music, Play } from 'lucide-vue-next'

interface Song {
  id: string
  title: string
  artist: string
  language: string
  mastery: number
}

defineProps<{
  song: Song
}>()

const emit = defineEmits<{
  practice: [songId: string]
}>()

const languageColors: Record<string, string> = {
  fr: 'bg-blue-500',
  en: 'bg-green-500',
  jp: 'bg-red-500',
  kr: 'bg-purple-500'
}
</script>

<template>
  <Card class="hover:shadow-lg transition-all cursor-pointer">
    <CardHeader class="flex flex-row items-center gap-2">
      <Music class="w-5 h-5 text-primary" />
      <div class="flex-1">
        <CardTitle class="text-lg">{{ song.title }}</CardTitle>
        <p class="text-sm text-muted-foreground">{{ song.artist }}</p>
      </div>
      <Badge :class="languageColors[song.language]">
        {{ song.language.toUpperCase() }}
      </Badge>
    </CardHeader>
    
    <CardContent class="space-y-2">
      <div class="flex items-center justify-between">
        <span class="text-sm">Maîtrise : {{ song.mastery }}%</span>
        <Button size="sm" @click="emit('practice', song.id)">
          <Play class="w-4 h-4 mr-2" />
          Pratiquer
        </Button>
      </div>
      
      <!-- Barre de progression -->
      <div class="w-full bg-gray-200 rounded-full h-2">
        <div 
          class="bg-primary h-2 rounded-full transition-all"
          :style="{ width: `${song.mastery}%` }"
        />
      </div>
    </CardContent>
  </Card>
</template>
```

### 2. Vue avec Tauri API

```typescript
// lib/tauri.ts
import { invoke } from '@tauri-apps/api/tauri'

export interface Song {
  id: string
  title: string
  artist: string
  language: string
  lyrics: string[]
  phonetic_lyrics?: string[]
}

export interface User {
  id: string
  username: string
  email: string
}

// Auth API
export const authApi = {
  async login(username: string, password: string): Promise<User> {
    return await invoke('login', { username, password })
  },
  
  async register(username: string, email: string, password: string): Promise<User> {
    return await invoke('register', { username, email, password })
  },
  
  async logout(): Promise<void> {
    return await invoke('logout')
  }
}

// Songs API
export const songsApi = {
  async getSongs(userId: string): Promise<Song[]> {
    return await invoke('get_songs', { userId })
  },
  
  async getSong(songId: string): Promise<Song> {
    return await invoke('get_song', { songId })
  },
  
  async createSong(data: {
    userId: string
    title: string
    artist: string
    language: string
    lyrics: string[]
  }): Promise<Song> {
    return await invoke('create_song', data)
  },
  
  async generatePhonetic(songId: string, language: string): Promise<string[]> {
    return await invoke('generate_phonetic', { songId, language })
  }
}

// Genius API
export const geniusApi = {
  async search(query: string): Promise<any[]> {
    return await invoke('genius_search', { query })
  },
  
  async import(songId: string, language: string): Promise<Song> {
    return await invoke('genius_import', { songId, language })
  }
}
```

### 3. Pinia Store (State Management)

```typescript
// stores/auth.ts
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi, type User } from '@/lib/tauri'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const isAuthenticated = computed(() => user.value !== null)
  
  async function login(username: string, password: string) {
    try {
      user.value = await authApi.login(username, password)
      return true
    } catch (error) {
      console.error('Login failed:', error)
      return false
    }
  }
  
  async function register(username: string, email: string, password: string) {
    try {
      user.value = await authApi.register(username, email, password)
      return true
    } catch (error) {
      console.error('Registration failed:', error)
      return false
    }
  }
  
  async function logout() {
    await authApi.logout()
    user.value = null
  }
  
  return {
    user,
    isAuthenticated,
    login,
    register,
    logout
  }
})
```

```typescript
// stores/songs.ts
import { defineStore } from 'pinia'
import { ref } from 'vue'
import { songsApi, type Song } from '@/lib/tauri'
import { useAuthStore } from './auth'

export const useSongsStore = defineStore('songs', () => {
  const songs = ref<Song[]>([])
  const currentSong = ref<Song | null>(null)
  const loading = ref(false)
  
  async function fetchSongs() {
    const authStore = useAuthStore()
    if (!authStore.user) return
    
    loading.value = true
    try {
      songs.value = await songsApi.getSongs(authStore.user.id)
    } finally {
      loading.value = false
    }
  }
  
  async function fetchSong(songId: string) {
    loading.value = true
    try {
      currentSong.value = await songsApi.getSong(songId)
    } finally {
      loading.value = false
    }
  }
  
  async function createSong(data: {
    title: string
    artist: string
    language: string
    lyrics: string[]
  }) {
    const authStore = useAuthStore()
    if (!authStore.user) return null
    
    const newSong = await songsApi.createSong({
      userId: authStore.user.id,
      ...data
    })
    
    songs.value.push(newSong)
    return newSong
  }
  
  return {
    songs,
    currentSong,
    loading,
    fetchSongs,
    fetchSong,
    createSong
  }
})
```

### 4. Vue Composable

```typescript
// composables/useSongs.ts
import { ref, computed } from 'vue'
import { useSongsStore } from '@/stores/songs'
import { storeToRefs } from 'pinia'

export function useSongs() {
  const songsStore = useSongsStore()
  const { songs, loading } = storeToRefs(songsStore)
  
  const songsByLanguage = computed(() => {
    const grouped: Record<string, typeof songs.value> = {}
    
    songs.value.forEach(song => {
      if (!grouped[song.language]) {
        grouped[song.language] = []
      }
      grouped[song.language].push(song)
    })
    
    return grouped
  })
  
  const searchSongs = (query: string) => {
    const lowerQuery = query.toLowerCase()
    return songs.value.filter(song => 
      song.title.toLowerCase().includes(lowerQuery) ||
      song.artist.toLowerCase().includes(lowerQuery)
    )
  }
  
  return {
    songs,
    loading,
    songsByLanguage,
    searchSongs,
    fetchSongs: songsStore.fetchSongs,
    createSong: songsStore.createSong
  }
}
```

### 5. Page Vue complète

```vue
<!-- views/SongsView.vue -->
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSongs } from '@/composables/useSongs'
import SongCard from '@/components/SongCard.vue'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { 
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select'
import { Search, Plus } from 'lucide-vue-next'
import { useRouter } from 'vue-router'

const router = useRouter()
const { songs, loading, songsByLanguage, searchSongs } = useSongs()

const searchQuery = ref('')
const selectedLanguage = ref<string>('all')

const filteredSongs = computed(() => {
  let result = searchQuery.value 
    ? searchSongs(searchQuery.value)
    : songs.value
  
  if (selectedLanguage.value !== 'all') {
    result = result.filter(s => s.language === selectedLanguage.value)
  }
  
  return result
})

const handlePractice = (songId: string) => {
  router.push(`/practice/${songId}`)
}

onMounted(async () => {
  await fetchSongs()
})
</script>

<template>
  <div class="container mx-auto p-6">
    <div class="flex items-center justify-between mb-6">
      <h1 class="text-3xl font-bold">Mes Chansons</h1>
      <Button @click="router.push('/songs/new')">
        <Plus class="w-4 h-4 mr-2" />
        Ajouter
      </Button>
    </div>
    
    <!-- Filters -->
    <div class="flex gap-4 mb-6">
      <div class="relative flex-1">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
        <Input
          v-model="searchQuery"
          placeholder="Rechercher une chanson..."
          class="pl-10"
        />
      </div>
      
      <Select v-model="selectedLanguage">
        <SelectTrigger class="w-[180px]">
          <SelectValue placeholder="Langue" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">Toutes</SelectItem>
          <SelectItem value="fr">Français</SelectItem>
          <SelectItem value="en">Anglais</SelectItem>
          <SelectItem value="jp">Japonais</SelectItem>
          <SelectItem value="kr">Coréen</SelectItem>
        </SelectContent>
      </Select>
    </div>
    
    <!-- Loading -->
    <div v-if="loading" class="text-center py-12">
      <p class="text-muted-foreground">Chargement...</p>
    </div>
    
    <!-- Empty state -->
    <div v-else-if="filteredSongs.length === 0" class="text-center py-12">
      <p class="text-muted-foreground mb-4">
        {{ searchQuery ? 'Aucune chanson trouvée' : 'Aucune chanson dans votre répertoire' }}
      </p>
      <Button @click="router.push('/songs/new')">
        Ajouter votre première chanson
      </Button>
    </div>
    
    <!-- Songs Grid -->
    <div 
      v-else
      class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4"
    >
      <SongCard
        v-for="song in filteredSongs"
        :key="song.id"
        :song="song"
        @practice="handlePractice"
      />
    </div>
  </div>
</template>
```

### 6. Mode Karaoke

```vue
<!-- components/KaraokeMode.vue -->
<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { Card, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Slider } from '@/components/ui/slider'
import { Play, Pause, SkipForward, SkipBack } from 'lucide-vue-next'
import type { Song } from '@/lib/tauri'

const props = defineProps<{
  song: Song
}>()

const currentLine = ref(0)
const isPaused = ref(false)
const speed = ref([3]) // seconds per line
let interval: number | null = null

const progress = computed(() => {
  return ((currentLine.value + 1) / props.song.lyrics.length) * 100
})

const start = () => {
  if (interval) return
  isPaused.value = false
  
  interval = setInterval(() => {
    if (currentLine.value < props.song.lyrics.length - 1) {
      currentLine.value++
    } else {
      stop()
    }
  }, speed.value[0] * 1000)
}

const pause = () => {
  if (interval) {
    clearInterval(interval)
    interval = null
  }
  isPaused.value = true
}

const togglePlayPause = () => {
  if (isPaused.value) {
    start()
  } else {
    pause()
  }
}

const next = () => {
  if (currentLine.value < props.song.lyrics.length - 1) {
    currentLine.value++
  }
}

const prev = () => {
  if (currentLine.value > 0) {
    currentLine.value--
  }
}

const stop = () => {
  pause()
  currentLine.value = 0
}

onMounted(() => {
  start()
})

onUnmounted(() => {
  if (interval) {
    clearInterval(interval)
  }
})
</script>

<template>
  <div class="min-h-screen bg-gradient-to-b from-purple-50 to-white dark:from-gray-900 dark:to-gray-800 p-6">
    <Card class="max-w-4xl mx-auto">
      <CardContent class="p-8">
        <!-- Progress -->
        <div class="mb-4">
          <div class="flex items-center justify-between mb-2">
            <span class="text-sm text-muted-foreground">
              Ligne {{ currentLine + 1 }} / {{ song.lyrics.length }}
            </span>
            <span class="text-sm text-muted-foreground">
              {{ Math.round(progress) }}%
            </span>
          </div>
          <div class="w-full bg-gray-200 rounded-full h-2">
            <div 
              class="bg-primary h-2 rounded-full transition-all"
              :style="{ width: `${progress}%` }"
            />
          </div>
        </div>
        
        <!-- Current Line -->
        <div class="text-center mb-8 min-h-[200px] flex flex-col items-center justify-center">
          <h2 class="text-4xl font-bold mb-4 animate-fade-in">
            {{ song.lyrics[currentLine] }}
          </h2>
          
          <p 
            v-if="song.phonetic_lyrics"
            class="text-xl text-muted-foreground"
          >
            {{ song.phonetic_lyrics[currentLine] }}
          </p>
        </div>
        
        <!-- Controls -->
        <div class="space-y-4">
          <div class="flex items-center justify-center gap-4">
            <Button 
              variant="outline" 
              size="icon"
              @click="prev"
              :disabled="currentLine === 0"
            >
              <SkipBack class="w-4 h-4" />
            </Button>
            
            <Button size="icon" @click="togglePlayPause">
              <Play v-if="isPaused" class="w-5 h-5" />
              <Pause v-else class="w-5 h-5" />
            </Button>
            
            <Button 
              variant="outline" 
              size="icon"
              @click="next"
              :disabled="currentLine === song.lyrics.length - 1"
            >
              <SkipForward class="w-4 h-4" />
            </Button>
          </div>
          
          <!-- Speed Control -->
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <span class="text-sm">Vitesse : {{ speed[0] }}s/ligne</span>
            </div>
            <Slider
              v-model="speed"
              :min="1"
              :max="10"
              :step="0.5"
            />
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</template>

<style scoped>
@keyframes fade-in {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-fade-in {
  animation: fade-in 0.3s ease-out;
}
</style>
```

---

## 🎨 Shadcn-vue vs Shadcn-ui (React)

| Feature | Shadcn-vue | Shadcn-ui |
|---------|------------|-----------|
| Framework | Vue 3 | React |
| Composants | 45+ | 50+ |
| Tailwind | ✅ | ✅ |
| TypeScript | ✅ | ✅ |
| Dark Mode | ✅ | ✅ |
| Accessible | ✅ | ✅ |
| Code ownership | ✅ | ✅ |

**Identique en qualité !** Juste adapté pour Vue.

---

## 🌐 i18n avec Vue

```bash
npm install vue-i18n@9
```

```typescript
// i18n.ts
import { createI18n } from 'vue-i18n'

const messages = {
  en: {
    practice: 'Practice',
    songs: 'Songs',
    addSong: 'Add Song',
  },
  fr: {
    practice: 'Pratiquer',
    songs: 'Chansons',
    addSong: 'Ajouter Chanson',
  },
  ko: {
    practice: '연습',
    songs: '노래',
    addSong: '노래 추가',
  },
  ja: {
    practice: '練習',
    songs: '曲',
    addSong: '曲を追加',
  }
}

export const i18n = createI18n({
  legacy: false,
  locale: 'fr',
  fallbackLocale: 'en',
  messages,
})
```

```vue
<script setup>
import { useI18n } from 'vue-i18n'

const { t, locale } = useI18n()
</script>

<template>
  <div>
    <h1>{{ t('songs') }}</h1>
    <Button>{{ t('addSong') }}</Button>
    
    <select v-model="locale">
      <option value="fr">Français</option>
      <option value="en">English</option>
      <option value="ko">한국어</option>
      <option value="ja">日本語</option>
    </select>
  </div>
</template>
```

---

## ✅ Avantages Vue + Tauri

### Vue 3 Composition API
- ✅ **Plus simple** que React hooks pour certains
- ✅ **Réactivité** automatique (ref, reactive)
- ✅ **Syntax concise** : `v-model`, `v-if`, `v-for`
- ✅ **Performance** excellente
- ✅ **TypeScript** support natif

### Écosystème Vue
- ✅ **Pinia** : State management simple et typé
- ✅ **Vue Router** : Routing officiel
- ✅ **Vite** : Build tool ultra rapide (même créateur que Vue)
- ✅ **Shadcn-vue** : Composants modernes
- ✅ **VueUse** : Utilities composables

---

## 🚀 Commencer Maintenant

```bash
# 1. Créer projet
npm create tauri-app
# Choisir Vue + TypeScript

cd lyremember

# 2. Installer Tailwind
npm install -D tailwindcss postcss autoprefixer
npx tailwindcss init -p

# 3. Installer Shadcn-vue
npx shadcn-vue@latest init

# 4. Installer dépendances supplémentaires
npm install pinia vue-router vue-i18n@9
npm install lucide-vue-next

# 5. Lancer en dev
npm run tauri dev
```

---

## 📊 Vue vs React avec Tauri

| Critère | Vue + Tauri | React + Tauri |
|---------|-------------|---------------|
| **Syntaxe** | ⭐⭐⭐ Template-based | ⭐⭐ JSX |
| **Courbe apprentissage** | ⭐⭐⭐ Plus facile | ⭐⭐ Moyen |
| **Performance** | ⭐⭐⭐ Excellent | ⭐⭐⭐ Excellent |
| **Réactivité** | ⭐⭐⭐ Automatique | ⭐⭐ Manuel |
| **TypeScript** | ⭐⭐⭐ Excellent | ⭐⭐⭐ Excellent |
| **Écosystème** | ⭐⭐ Bon | ⭐⭐⭐ Énorme |
| **Tauri Support** | ⭐⭐⭐ Parfait | ⭐⭐⭐ Parfait |
| **UI Libs** | ⭐⭐ Shadcn-vue | ⭐⭐⭐ Shadcn-ui |

---

## ✅ Verdict Final

### **Pour vous : Tauri + Vue 3 + Shadcn-vue** 🎯

**Stack complète :**
```
Frontend : Vue 3 + TypeScript + Vite
UI : Shadcn-vue + Tailwind CSS
Icons : Lucide Vue
State : Pinia
Router : Vue Router
i18n : Vue I18n
Desktop : Tauri (Rust)
Mobile : PWA ou Tauri Mobile (beta)
Backend : Rust (Tauri commands)
BDD : SQLite (rusqlite)
```

**Pourquoi c'est parfait pour vous :**
- ✅ **Tauri** : Performance native, léger
- ✅ **Vue** : Vous connaissez déjà !
- ✅ **Shadcn-vue** : Composants modernes comme Shadcn-ui
- ✅ **TypeScript** : Type safety
- ✅ **Gratuit** : Tout open-source
- ✅ **Desktop + Mobile** : Un codebase

**Prêt à commencer ?** 🚀

Je peux vous aider à :
1. Setup le projet Tauri + Vue
2. Configurer Shadcn-vue + Tailwind
3. Créer les premiers composants
4. Implémenter le backend Rust

**Dites-moi et on y va !** 😊
