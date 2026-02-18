<script setup lang="ts">
import { ref, onMounted } from "vue";
import * as api from './lib/tauri-api';

const status = ref("");
const logs = ref<string[]>([]);
const loading = ref(false);

function addLog(message: string) {
  logs.value.unshift(`[${new Date().toLocaleTimeString()}] ${message}`);
  console.log(message);
}

async function testIntegration() {
  loading.value = true;
  logs.value = [];
  
  try {
    // Test health check
    addLog("🔍 Testing health check...");
    const health = await api.healthCheck();
    addLog(`✅ Health check: ${health}`);
    
    // Test registration
    addLog("👤 Testing user registration...");
    const user = await api.register(
      `user_${Date.now()}`,
      `test@example.com`,
      "password123"
    );
    addLog(`✅ User registered: ${user.username} (ID: ${user.id})`);
    
    // Test login
    addLog("🔐 Testing login...");
    const token = await api.login(user.username, "password123");
    addLog(`✅ Login successful, token: ${token.substring(0, 20)}...`);
    
    // Test song creation
    addLog("🎵 Testing song creation with phonetics...");
    const song = await api.createSong(
      "千本桜",
      "初音ミク",
      "jp",
      ["千本桜", "夜ニ紛レ", "君ノ声モ届カナイヨ"],
      true
    );
    addLog(`✅ Song created: ${song.title} by ${song.artist}`);
    if (song.phonetic_lyrics) {
      addLog(`   📝 Phonetic: ${song.phonetic_lyrics.join(", ")}`);
    }
    if (song.translations) {
      addLog(`   🌐 Translation available: ${Object.keys(song.translations).join(", ")}`);
    }
    
    // Test adding to repertoire
    addLog("📚 Adding song to user's repertoire...");
    await api.addToRepertoire(user.id, song.id);
    addLog("✅ Song added to repertoire");
    
    // Test getting user songs
    addLog("📖 Getting user's songs...");
    const userSongs = await api.getUserSongs(user.id);
    addLog(`✅ User has ${userSongs.length} songs`);
    
    // Test practice session
    addLog("🎮 Creating practice session...");
    const session = await api.createPracticeSession(
      user.id,
      song.id,
      "karaoke",
      85.5,
      3,
      2,
      120
    );
    addLog(`✅ Practice session created: ${session.mode}, score: ${session.score}%`);
    
    // Test stats
    addLog("📊 Getting user statistics...");
    const stats = await api.getUserStats(user.id);
    addLog(`✅ Stats: ${stats.total_sessions} sessions, avg score: ${stats.average_score.toFixed(1)}%`);
    
    status.value = "✅ All tests passed!";
    addLog("🎉 Integration test completed successfully!");
    
  } catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error);
    status.value = `❌ Error: ${errorMsg}`;
    addLog(`❌ Error: ${errorMsg}`);
  } finally {
    loading.value = false;
  }
}

onMounted(() => {
  addLog("🚀 LyRemember - Backend Integration Test Ready");
  addLog("Click 'Run Integration Test' to test all backend features");
});
</script>

<template>
  <main class="container">
    <h1>🎵 LyRemember - Backend Integration Test</h1>
    
    <div class="test-section">
      <button @click="testIntegration" :disabled="loading" class="test-button">
        {{ loading ? '⏳ Running Tests...' : '▶️ Run Integration Test' }}
      </button>
      
      <div v-if="status" class="status">
        {{ status }}
      </div>
    </div>
    
    <div class="logs-section">
      <h2>📝 Test Logs</h2>
      <div class="logs">
        <div v-for="(log, index) in logs" :key="index" class="log-entry">
          {{ log }}
        </div>
        <div v-if="logs.length === 0" class="no-logs">
          No logs yet. Run the test to see backend integration in action!
        </div>
      </div>
    </div>
    
    <div class="info-section">
      <h3>🔧 What This Tests</h3>
      <ul>
        <li>✅ Health check (backend connection)</li>
        <li>✅ User registration (bcrypt + SQLite)</li>
        <li>✅ User login (JWT tokens)</li>
        <li>✅ Song creation with auto phonetic (PyO3 + pykakasi)</li>
        <li>✅ Song creation with auto translation (LibreTranslate)</li>
        <li>✅ Add song to repertoire (many-to-many)</li>
        <li>✅ Practice session tracking</li>
        <li>✅ User statistics aggregation</li>
      </ul>
    </div>
  </main>
</template>

<style scoped>
.container {
  max-width: 900px;
  margin: 0 auto;
  padding: 2rem;
}

h1 {
  text-align: center;
  margin-bottom: 2rem;
  color: #24c8db;
}

.test-section {
  text-align: center;
  margin: 2rem 0;
}

.test-button {
  font-size: 1.2rem;
  padding: 1rem 2rem;
  background: linear-gradient(135deg, #24c8db 0%, #396cd8 100%);
  color: white;
  border: none;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s;
  font-weight: 600;
}

.test-button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(36, 200, 219, 0.4);
}

.test-button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.status {
  margin-top: 1rem;
  padding: 1rem;
  border-radius: 8px;
  font-size: 1.1rem;
  font-weight: 600;
  background: rgba(36, 200, 219, 0.1);
  border: 2px solid #24c8db;
}

.logs-section {
  margin: 2rem 0;
}

.logs-section h2 {
  margin-bottom: 1rem;
  color: #396cd8;
}

.logs {
  background: #1a1a1a;
  border-radius: 8px;
  padding: 1rem;
  max-height: 400px;
  overflow-y: auto;
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
}

.log-entry {
  padding: 0.5rem;
  margin-bottom: 0.25rem;
  border-left: 3px solid #24c8db;
  background: rgba(36, 200, 219, 0.05);
  color: #f6f6f6;
}

.no-logs {
  color: #888;
  font-style: italic;
  text-align: center;
  padding: 2rem;
}

.info-section {
  margin-top: 2rem;
  padding: 1.5rem;
  background: rgba(57, 108, 216, 0.1);
  border-radius: 8px;
  border: 2px solid #396cd8;
}

.info-section h3 {
  margin-bottom: 1rem;
  color: #396cd8;
}

.info-section ul {
  list-style: none;
  padding: 0;
}

.info-section li {
  padding: 0.5rem 0;
  border-bottom: 1px solid rgba(57, 108, 216, 0.2);
}

.info-section li:last-child {
  border-bottom: none;
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>