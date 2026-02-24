import { useState } from "react";

const PALETTE = {
  bgDeep: "#0D0B1A",
  bgCard: "#1A1528",
  bgCardHover: "#231D35",
  accentGold: "#F2A93B",
  accentViolet: "#7B6FA0",
  accentVioletMuted: "#5E5480",
  textPrimary: "#F5F0EB",
  textSecondary: "#B8B0D0",
  textMuted: "#8A82A0",
  surface: "#131022",
  border: "#2A2440",
  success: "#30A46C",
  successMuted: "#1A3A2A",
};

const LyreSvg = ({ size = 22, color = "#F2A93B" }) => (
  <svg width={size} height={size} viewBox="0 0 48 48" fill="none">
    {/* Branches lyre grecque */}
    <path d="M18 44C16 38 14 32 13 28C11 22 10 16 12 10C14 6 16 4 14 2" stroke={color} strokeWidth="1.8" strokeLinecap="round"/>
    <path d="M30 44C32 38 34 32 35 28C37 22 38 16 36 10C34 6 32 4 34 2" stroke={color} strokeWidth="1.8" strokeLinecap="round"/>
    <line x1="18" y1="44" x2="30" y2="44" stroke={color} strokeWidth="1.8" strokeLinecap="round"/>
    <path d="M13.5 12H34.5" stroke={color} strokeWidth="1.5" strokeLinecap="round"/>
    {/* Cordes = lignes de texte brisées */}
    <line x1="20" y1="16" x2="20" y2="20" stroke={color} strokeWidth="1.1" strokeLinecap="round"/>
    <line x1="20" y1="23" x2="20" y2="30" stroke={color} strokeWidth="1.1" strokeLinecap="round"/>
    <line x1="20" y1="33" x2="20" y2="36" stroke={color} strokeWidth="1.1" strokeLinecap="round"/>
    <line x1="24" y1="14" x2="24" y2="22" stroke={color} strokeWidth="1.1" strokeLinecap="round"/>
    <line x1="24" y1="25" x2="24" y2="32" stroke={color} strokeWidth="1.1" strokeLinecap="round"/>
    <line x1="24" y1="35" x2="24" y2="42" stroke={color} strokeWidth="1.1" strokeLinecap="round"/>
    <line x1="28" y1="16" x2="28" y2="24" stroke={color} strokeWidth="1.1" strokeLinecap="round"/>
    <line x1="28" y1="27" x2="28" y2="34" stroke={color} strokeWidth="1.1" strokeLinecap="round"/>
    <line x1="28" y1="37" x2="28" y2="40" stroke={color} strokeWidth="1.1" strokeLinecap="round"/>
  </svg>
);

// Mock data
const mockSongs = [
  { id: 1, title: "Something About Us", artist: "Daft Punk", lang: "EN", progress: 72, cover: "🌙", tags: ["Électro", "Mélancolique"] },
  { id: 2, title: "Papaoutai", artist: "Stromae", lang: "FR", progress: 45, cover: "🎭", tags: ["Pop", "Engagé"] },
  { id: 3, title: "99 Luftballons", artist: "Nena", lang: "DE", progress: 18, cover: "🎈", tags: ["Pop", "80s"] },
  { id: 4, title: "La Tortura", artist: "Shakira", lang: "ES", progress: 60, cover: "🔥", tags: ["Latin", "Dansant"] },
  { id: 5, title: "Nuovo Cinema Paradiso", artist: "Ennio Morricone", lang: "IT", progress: 30, cover: "🎬", tags: ["Cinéma", "Émouvant"] },
];

const langFlags = { EN: "🇬🇧", FR: "🇫🇷", DE: "🇩🇪", ES: "🇪🇸", IT: "🇮🇹", JP: "🇯🇵", PT: "🇧🇷" };

const mockLyrics = [
  { original: "It might not be the right time", translation: "Ce n'est peut-être pas le bon moment" },
  { original: "I might not be the right one", translation: "Je ne suis peut-être pas la bonne personne" },
  { original: "But there's something about us I want to say", translation: "Mais il y a quelque chose entre nous que je veux dire" },
  { original: "'Cause there's something between us anyway", translation: "Car il y a quelque chose entre nous de toute façon" },
  { original: "", translation: "", section: "Verse 2" },
  { original: "I might not be the right one", translation: "Je ne suis peut-être pas la bonne personne" },
  { original: "It might not be the right time", translation: "Ce n'est peut-être pas le bon moment" },
  { original: "But there's something about us I've got to do", translation: "Mais il y a quelque chose entre nous que je dois faire" },
  { original: "Some kind of secret I will share with you", translation: "Une sorte de secret que je partagerai avec toi" },
];

// ─── PHONE FRAME ─────────────────────────────────────────
function PhoneFrame({ children, title, activeTab, onTabChange, showFab, onFabClick }) {
  return (
    <div style={{ display: "flex", flexDirection: "column", alignItems: "center" }}>
      <div style={{
        width: 375,
        height: 740,
        background: PALETTE.bgDeep,
        borderRadius: 40,
        border: `3px solid ${PALETTE.border}`,
        overflow: "hidden",
        position: "relative",
        boxShadow: `0 0 60px rgba(123, 111, 160, 0.06), 0 20px 40px rgba(0,0,0,0.5)`,
        display: "flex",
        flexDirection: "column",
      }}>
        {/* Status bar */}
        <div style={{ height: 44, display: "flex", alignItems: "center", justifyContent: "center", flexShrink: 0 }}>
          <div style={{ width: 120, height: 28, background: PALETTE.bgDeep, borderRadius: "0 0 20px 20px", border: `1px solid ${PALETTE.border}`, borderTop: "none" }} />
        </div>

        {/* Content */}
        <div style={{ flex: 1, overflowY: "auto", overflowX: "hidden" }}>
          {children}
        </div>

        {/* Bottom nav */}
        {activeTab !== undefined && (
          <div style={{
            height: 68,
            background: PALETTE.surface,
            borderTop: `1px solid ${PALETTE.border}`,
            display: "flex",
            alignItems: "center",
            justifyContent: "space-around",
            padding: "0 24px",
            flexShrink: 0,
            position: "relative",
          }}>
            {/* Ma Lyre tab */}
            <button onClick={() => onTabChange?.("lyre")} style={{
              background: "none", border: "none", cursor: "pointer",
              display: "flex", flexDirection: "column", alignItems: "center", gap: 4, padding: 8,
            }}>
              <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke={activeTab === "lyre" ? PALETTE.accentGold : PALETTE.textMuted} strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round">
                <path d="M2 4C4 3 7 3 9 4C11 5 12 5 12 5V21C12 21 11 20 9 19.5C7 19 4 19 2 20V4Z"/>
                <path d="M22 4C20 3 17 3 15 4C13 5 12 5 12 5V21C12 21 13 20 15 19.5C17 19 20 19 22 20V4Z"/>
                <line x1="5" y1="8" x2="9" y2="8" opacity="0.5"/>
                <line x1="5" y1="11" x2="9" y2="11" opacity="0.5"/>
                <line x1="15" y1="8" x2="19" y2="8" opacity="0.5"/>
                <line x1="15" y1="11" x2="19" y2="11" opacity="0.5"/>
              </svg>
              <span style={{
                fontSize: 10, fontFamily: "'DM Sans', sans-serif",
                color: activeTab === "lyre" ? PALETTE.accentGold : PALETTE.textMuted,
                fontWeight: activeTab === "lyre" ? 600 : 400,
              }}>Ma Lyre</span>
            </button>

            {/* FAB - Ajouter */}
            <button onClick={() => onFabClick?.()} style={{
              width: 52, height: 52, borderRadius: 16,
              background: `linear-gradient(135deg, ${PALETTE.accentGold}, #E09520)`,
              border: "none", cursor: "pointer",
              display: "flex", alignItems: "center", justifyContent: "center",
              boxShadow: `0 4px 20px ${PALETTE.accentGold}40`,
              position: "relative", top: -14,
            }}>
              <span style={{ fontSize: 26, color: PALETTE.bgDeep, fontWeight: 300, lineHeight: 1 }}>+</span>
            </button>

            {/* Mémoriser tab */}
            <button onClick={() => onTabChange?.("memorize")} style={{
              background: "none", border: "none", cursor: "pointer",
              display: "flex", flexDirection: "column", alignItems: "center", gap: 4, padding: 8,
            }}>
              <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke={activeTab === "memorize" ? PALETTE.accentGold : PALETTE.textMuted} strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round">
                <path d="M4 4H9C9 4 9 2 10.5 2C12 2 12 4 12 4H18V9C18 9 20 9 20 10.5C20 12 18 12 18 12V18H12C12 18 12 20 10.5 20C9 20 9 18 9 18H4V12C4 12 2 12 2 10.5C2 9 4 9 4 9V4Z"/>
              </svg>
              <span style={{
                fontSize: 10, fontFamily: "'DM Sans', sans-serif",
                color: activeTab === "memorize" ? PALETTE.accentGold : PALETTE.textMuted,
                fontWeight: activeTab === "memorize" ? 600 : 400,
              }}>Mémoriser</span>
            </button>
          </div>
        )}
      </div>
      {title && <p style={{ color: PALETTE.textSecondary, fontSize: 13, marginTop: 16, fontFamily: "'DM Sans', sans-serif" }}>{title}</p>}
    </div>
  );
}

// ─── MA LYRE (Collection + Comprendre) ─────────────────────
function MaLyreScreen() {
  const [selectedSong, setSelectedSong] = useState(null);
  const [targetLang, setTargetLang] = useState("FR");

  if (selectedSong) {
    return (
      <div>
        {/* Song header */}
        <div style={{
          padding: "12px 20px",
          display: "flex", alignItems: "center", gap: 12,
          borderBottom: `1px solid ${PALETTE.border}`,
        }}>
          <button onClick={() => setSelectedSong(null)} style={{
            background: "none", border: "none", color: PALETTE.textSecondary, fontSize: 20, cursor: "pointer", padding: 0,
          }}>‹</button>
          <div style={{
            width: 40, height: 40, borderRadius: 10,
            background: `linear-gradient(135deg, ${PALETTE.accentViolet}40, ${PALETTE.bgCard})`,
            display: "flex", alignItems: "center", justifyContent: "center", fontSize: 20,
          }}>{selectedSong.cover}</div>
          <div style={{ flex: 1 }}>
            <div style={{ color: PALETTE.textPrimary, fontSize: 15, fontWeight: 600, fontFamily: "'DM Sans', sans-serif" }}>{selectedSong.title}</div>
            <div style={{ color: PALETTE.textSecondary, fontSize: 12, fontFamily: "'DM Sans', sans-serif" }}>{selectedSong.artist}</div>
          </div>
          <span style={{ fontSize: 12, color: PALETTE.textMuted }}>⋯</span>
        </div>

        {/* Language selector */}
        <div style={{
          padding: "14px 20px",
          display: "flex", alignItems: "center", gap: 10,
          borderBottom: `1px solid ${PALETTE.border}`,
        }}>
          <span style={{ color: PALETTE.textMuted, fontSize: 12, fontFamily: "'DM Sans', sans-serif" }}>Traduire en :</span>
          <div style={{ display: "flex", gap: 6 }}>
            {["FR", "ES", "DE", "JP"].map((lang) => (
              <button key={lang} onClick={() => setTargetLang(lang)} style={{
                background: targetLang === lang ? `${PALETTE.accentGold}25` : PALETTE.bgCard,
                border: `1px solid ${targetLang === lang ? PALETTE.accentGold : PALETTE.border}`,
                borderRadius: 8, padding: "6px 10px", cursor: "pointer",
                color: targetLang === lang ? PALETTE.accentGold : PALETTE.textSecondary,
                fontSize: 13, fontFamily: "'DM Sans', sans-serif",
              }}>{langFlags[lang]} {lang}</button>
            ))}
          </div>
        </div>

        {/* Section label */}
        <div style={{ padding: "16px 20px 8px" }}>
          <span style={{
            color: PALETTE.accentGold, fontSize: 11, letterSpacing: 2, textTransform: "uppercase",
            fontFamily: "'DM Sans', sans-serif",
          }}>Verse 1</span>
        </div>

        {/* Lyrics bilingual */}
        <div style={{ padding: "0 20px 20px" }}>
          {mockLyrics.map((line, i) => {
            if (line.section) {
              return (
                <div key={i} style={{ padding: "16px 0 8px" }}>
                  <span style={{
                    color: PALETTE.accentGold, fontSize: 11, letterSpacing: 2, textTransform: "uppercase",
                    fontFamily: "'DM Sans', sans-serif",
                  }}>{line.section}</span>
                </div>
              );
            }
            return (
              <div key={i} style={{
                marginBottom: 16,
                paddingLeft: 12,
                borderLeft: `2px solid ${PALETTE.accentViolet}40`,
              }}>
                <p style={{
                  color: PALETTE.textPrimary, fontSize: 15, margin: 0, lineHeight: 1.5,
                  fontFamily: "'Cormorant Garamond', serif", fontWeight: 500,
                }}>{line.original}</p>
                <p style={{
                  color: PALETTE.accentGold, fontSize: 13, margin: "4px 0 0", lineHeight: 1.4,
                  fontFamily: "'DM Sans', sans-serif", fontStyle: "italic", opacity: 0.85,
                }}>{line.translation}</p>
              </div>
            );
          })}
        </div>
      </div>
    );
  }

  return (
    <div style={{ padding: "8px 20px 20px" }}>
      {/* Header */}
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: 20 }}>
        <div>
          <h1 style={{ color: PALETTE.textPrimary, fontFamily: "'Cormorant Garamond', serif", fontSize: 28, margin: 0, fontWeight: 400 }}>
            Ma <span style={{ color: PALETTE.accentGold }}>Lyre</span>
          </h1>
          <p style={{ color: PALETTE.textMuted, fontSize: 12, margin: "4px 0 0", fontFamily: "'DM Sans', sans-serif" }}>{mockSongs.length} chansons · 4 langues</p>
        </div>
        <div style={{
          width: 36, height: 36, borderRadius: 12,
          background: PALETTE.bgCard, border: `1px solid ${PALETTE.border}`,
          display: "flex", alignItems: "center", justifyContent: "center",
        }}>
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke={PALETTE.textSecondary} strokeWidth="2" strokeLinecap="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
        </div>
      </div>

      {/* Language filter pills */}
      <div style={{ display: "flex", gap: 8, marginBottom: 22, overflowX: "auto" }}>
        {[{ label: "Tout", active: true }, { label: "🇬🇧 EN", active: false }, { label: "🇫🇷 FR", active: false }, { label: "🇩🇪 DE", active: false }, { label: "🇪🇸 ES", active: false }].map((f) => (
          <div key={f.label} style={{
            background: f.active ? PALETTE.accentGold : PALETTE.bgCard,
            color: f.active ? PALETTE.bgDeep : PALETTE.textSecondary,
            borderRadius: 20, padding: "7px 16px", fontSize: 13, whiteSpace: "nowrap",
            fontWeight: f.active ? 600 : 400, fontFamily: "'DM Sans', sans-serif",
            border: f.active ? "none" : `1px solid ${PALETTE.border}`,
          }}>{f.label}</div>
        ))}
      </div>

      {/* Song list */}
      {mockSongs.map((song) => (
        <button key={song.id} onClick={() => setSelectedSong(song)} style={{
          background: PALETTE.bgCard,
          borderRadius: 16, padding: 14, marginBottom: 10,
          display: "flex", gap: 14, alignItems: "center",
          border: `1px solid ${PALETTE.border}`,
          width: "100%", cursor: "pointer", textAlign: "left",
        }}>
          <div style={{
            width: 50, height: 50, borderRadius: 12,
            background: `linear-gradient(135deg, ${PALETTE.accentViolet}30, ${PALETTE.bgDeep})`,
            display: "flex", alignItems: "center", justifyContent: "center",
            fontSize: 24, flexShrink: 0,
          }}>{song.cover}</div>
          <div style={{ flex: 1, minWidth: 0 }}>
            <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
              <span style={{ color: PALETTE.textPrimary, fontSize: 14, fontWeight: 600, fontFamily: "'DM Sans', sans-serif" }}>{song.title}</span>
              <span style={{
                fontSize: 11, color: PALETTE.textMuted, background: PALETTE.surface,
                padding: "1px 6px", borderRadius: 4, fontFamily: "'DM Sans', sans-serif",
              }}>{langFlags[song.lang]}</span>
            </div>
            <div style={{ color: PALETTE.textSecondary, fontSize: 12, fontFamily: "'DM Sans', sans-serif", marginTop: 2 }}>{song.artist}</div>
            {/* Mini progress bar */}
            <div style={{ display: "flex", alignItems: "center", gap: 8, marginTop: 6 }}>
              <div style={{ flex: 1, height: 3, background: PALETTE.border, borderRadius: 2 }}>
                <div style={{ width: `${song.progress}%`, height: "100%", background: PALETTE.accentGold, borderRadius: 2 }} />
              </div>
              <span style={{ color: PALETTE.textMuted, fontSize: 10, fontFamily: "'DM Sans', sans-serif" }}>{song.progress}%</span>
            </div>
          </div>
        </button>
      ))}
    </div>
  );
}

// ─── AJOUTER (FAB Screen) ─────────────────────────────────
function AjouterScreen({ onClose }) {
  return (
    <div style={{ padding: "8px 20px 20px" }}>
      {/* Header */}
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: 28 }}>
        <button onClick={onClose} style={{ background: "none", border: "none", color: PALETTE.textSecondary, fontSize: 16, cursor: "pointer", padding: 0 }}>✕</button>
        <p style={{ color: PALETTE.textPrimary, fontSize: 16, fontFamily: "'DM Sans', sans-serif", fontWeight: 600, margin: 0 }}>Ajouter une chanson</p>
        <span style={{ width: 16 }} />
      </div>

      {/* Search song */}
      <div style={{
        background: PALETTE.bgCard, borderRadius: 14, padding: "14px 16px",
        display: "flex", alignItems: "center", gap: 10, marginBottom: 20,
        border: `1px solid ${PALETTE.accentGold}40`,
      }}>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke={PALETTE.accentGold} strokeWidth="2" strokeLinecap="round"><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg>
        <span style={{ color: PALETTE.textMuted, fontSize: 14, fontFamily: "'DM Sans', sans-serif" }}>Rechercher un titre ou un artiste...</span>
      </div>

      {/* Fake search results */}
      <p style={{ color: PALETTE.textMuted, fontSize: 11, letterSpacing: 2, textTransform: "uppercase", margin: "0 0 12px", fontFamily: "'DM Sans', sans-serif" }}>Résultats</p>
      {[
        { title: "Bohemian Rhapsody", artist: "Queen", lang: "EN" },
        { title: "Bohemian Like You", artist: "The Dandy Warhols", lang: "EN" },
      ].map((r, i) => (
        <div key={i} style={{
          background: PALETTE.bgCard, borderRadius: 12, padding: 14,
          display: "flex", alignItems: "center", gap: 12, marginBottom: 8,
          border: `1px solid ${PALETTE.border}`,
        }}>
          <div style={{
            width: 42, height: 42, borderRadius: 10,
            background: `linear-gradient(135deg, ${PALETTE.accentViolet}25, ${PALETTE.bgDeep})`,
            display: "flex", alignItems: "center", justifyContent: "center", fontSize: 18,
          }}>🎵</div>
          <div style={{ flex: 1 }}>
            <div style={{ color: PALETTE.textPrimary, fontSize: 14, fontWeight: 600, fontFamily: "'DM Sans', sans-serif" }}>{r.title}</div>
            <div style={{ color: PALETTE.textSecondary, fontSize: 12, fontFamily: "'DM Sans', sans-serif" }}>{r.artist} · {langFlags[r.lang]} {r.lang}</div>
          </div>
          <div style={{
            background: `${PALETTE.accentGold}20`, borderRadius: 8,
            padding: "6px 12px", color: PALETTE.accentGold, fontSize: 12,
            fontFamily: "'DM Sans', sans-serif", fontWeight: 600,
          }}>+ Ajouter</div>
        </div>
      ))}

      {/* Divider */}
      <div style={{ display: "flex", alignItems: "center", gap: 12, margin: "24px 0" }}>
        <div style={{ flex: 1, height: 1, background: PALETTE.border }} />
        <span style={{ color: PALETTE.textMuted, fontSize: 12, fontFamily: "'DM Sans', sans-serif" }}>ou saisir manuellement</span>
        <div style={{ flex: 1, height: 1, background: PALETTE.border }} />
      </div>

      {/* Manual fields */}
      {[
        { label: "Titre", placeholder: "Nom de la chanson" },
        { label: "Artiste", placeholder: "Nom de l'artiste" },
        { label: "Langue originale", placeholder: "Anglais, Espagnol, Allemand..." },
      ].map((field) => (
        <div key={field.label} style={{ marginBottom: 14 }}>
          <label style={{ color: PALETTE.textSecondary, fontSize: 12, fontFamily: "'DM Sans', sans-serif", marginBottom: 6, display: "block" }}>{field.label}</label>
          <div style={{
            background: PALETTE.bgCard, borderRadius: 12, padding: "13px 16px",
            border: `1px solid ${PALETTE.border}`,
          }}>
            <span style={{ color: PALETTE.textMuted, fontSize: 14, fontFamily: "'DM Sans', sans-serif" }}>{field.placeholder}</span>
          </div>
        </div>
      ))}

      {/* Lyrics input */}
      <div style={{ marginBottom: 14 }}>
        <label style={{ color: PALETTE.textSecondary, fontSize: 12, fontFamily: "'DM Sans', sans-serif", marginBottom: 6, display: "block" }}>Paroles</label>
        <div style={{
          background: PALETTE.bgCard, borderRadius: 12, padding: "13px 16px",
          minHeight: 90, border: `1px solid ${PALETTE.border}`,
        }}>
          <span style={{ color: PALETTE.textMuted, fontSize: 14, fontFamily: "'DM Sans', sans-serif" }}>Coller ou saisir les paroles originales...</span>
        </div>
      </div>

      {/* Context note */}
      <div style={{ marginBottom: 20 }}>
        <label style={{ color: PALETTE.textSecondary, fontSize: 12, fontFamily: "'DM Sans', sans-serif", marginBottom: 6, display: "block" }}>Note personnelle <span style={{ color: PALETTE.textMuted }}>(optionnel)</span></label>
        <div style={{
          background: PALETTE.bgCard, borderRadius: 12, padding: "13px 16px",
          border: `1px solid ${PALETTE.border}`,
        }}>
          <span style={{ color: PALETTE.textMuted, fontSize: 14, fontFamily: "'DM Sans', sans-serif" }}>Où l'avez-vous découverte ?</span>
        </div>
      </div>

      {/* Save button */}
      <div style={{
        background: PALETTE.accentGold, borderRadius: 14, padding: 16,
        textAlign: "center", color: PALETTE.bgDeep, fontWeight: 700, fontSize: 15,
        fontFamily: "'DM Sans', sans-serif",
      }}>Sauvegarder dans ma Lyre</div>
    </div>
  );
}

// ─── MEMORISER ─────────────────────────────────────────
function MemoriserScreen() {
  const [selectedSong, setSelectedSong] = useState(null);
  const [selectedMethod, setSelectedMethod] = useState(null);

  const methods = [
    { id: "trous", icon: "▢", name: "Texte à trous", desc: "Des mots sont masqués, retrouvez-les", color: "#F59E0B" },
    { id: "premiere", icon: "A_", name: "Première lettre", desc: "Seule la première lettre est visible", color: "#10B981" },
    { id: "karaoké", icon: "♪→", name: "Karaoké", desc: "Les paroles défilent, suivez le rythme", color: "#EC4899" },
    { id: "flash", icon: "⚡", name: "Flashcards", desc: "Vers par vers, testez votre mémoire", color: "#6366F1" },
  ];

  // Exercise: Texte à trous
  if (selectedMethod === "trous" && selectedSong) {
    const exerciseLines = [
      { text: "It might not be the right ____", missing: "time", revealed: false },
      { text: "I might not be the ____ one", missing: "right", revealed: true },
      { text: "But there's ____ about us I want to say", missing: "something", revealed: false },
      { text: "'Cause there's something ____ us anyway", missing: "between", revealed: false },
    ];
    return (
      <div style={{ padding: "8px 20px 20px" }}>
        <div style={{ display: "flex", alignItems: "center", gap: 12, marginBottom: 24 }}>
          <button onClick={() => setSelectedMethod(null)} style={{ background: "none", border: "none", color: PALETTE.textSecondary, fontSize: 20, cursor: "pointer", padding: 0 }}>‹</button>
          <div style={{ flex: 1 }}>
            <div style={{ color: PALETTE.textPrimary, fontSize: 15, fontWeight: 600, fontFamily: "'DM Sans', sans-serif" }}>Texte à trous</div>
            <div style={{ color: PALETTE.textSecondary, fontSize: 12, fontFamily: "'DM Sans', sans-serif" }}>{selectedSong.title} · {selectedSong.artist}</div>
          </div>
          <span style={{
            background: `${PALETTE.accentGold}20`, color: PALETTE.accentGold,
            fontSize: 12, padding: "4px 10px", borderRadius: 8, fontFamily: "'DM Sans', sans-serif",
          }}>Verse 1</span>
        </div>

        {exerciseLines.map((line, i) => (
          <div key={i} style={{
            background: PALETTE.bgCard, borderRadius: 14, padding: 16, marginBottom: 10,
            border: `1px solid ${line.revealed ? PALETTE.success + "40" : PALETTE.border}`,
          }}>
            <p style={{
              color: PALETTE.textPrimary, fontSize: 16, margin: 0, lineHeight: 1.6,
              fontFamily: "'Cormorant Garamond', serif",
            }}>
              {line.text.split("____").map((part, j, arr) => (
                <span key={j}>
                  {part}
                  {j < arr.length - 1 && (
                    line.revealed ? (
                      <span style={{
                        color: PALETTE.success, fontWeight: 600,
                        background: PALETTE.successMuted, padding: "2px 8px", borderRadius: 6,
                      }}>{line.missing}</span>
                    ) : (
                      <span style={{
                        display: "inline-block", width: 80, height: 24,
                        background: `${PALETTE.accentGold}15`, borderRadius: 6,
                        border: `1px dashed ${PALETTE.accentGold}50`,
                        verticalAlign: "middle",
                      }} />
                    )
                  )}
                </span>
              ))}
            </p>
            {line.revealed && (
              <p style={{
                color: PALETTE.accentGold, fontSize: 12, margin: "8px 0 0",
                fontFamily: "'DM Sans', sans-serif", fontStyle: "italic", opacity: 0.8,
              }}>Ce n'est peut-être pas la bonne personne</p>
            )}
          </div>
        ))}

        {/* Word bank */}
        <p style={{ color: PALETTE.textMuted, fontSize: 11, letterSpacing: 2, textTransform: "uppercase", margin: "20px 0 10px", fontFamily: "'DM Sans', sans-serif" }}>Mots disponibles</p>
        <div style={{ display: "flex", gap: 8, flexWrap: "wrap" }}>
          {["time", "something", "between", "say"].map((word) => (
            <div key={word} style={{
              background: PALETTE.bgCard, border: `1px solid ${PALETTE.accentGold}40`,
              borderRadius: 10, padding: "8px 16px",
              color: PALETTE.accentGold, fontSize: 14, fontFamily: "'DM Sans', sans-serif",
            }}>{word}</div>
          ))}
        </div>

        {/* Progress */}
        <div style={{
          marginTop: 24, background: PALETTE.bgCard, borderRadius: 14, padding: 16,
          border: `1px solid ${PALETTE.border}`, display: "flex", alignItems: "center", gap: 14,
        }}>
          <div style={{
            width: 44, height: 44, borderRadius: 22,
            background: `conic-gradient(${PALETTE.accentGold} 25%, ${PALETTE.border} 25%)`,
            display: "flex", alignItems: "center", justifyContent: "center",
          }}>
            <div style={{
              width: 34, height: 34, borderRadius: 17, background: PALETTE.bgCard,
              display: "flex", alignItems: "center", justifyContent: "center",
              color: PALETTE.accentGold, fontSize: 11, fontWeight: 700, fontFamily: "'DM Sans', sans-serif",
            }}>1/4</div>
          </div>
          <div>
            <div style={{ color: PALETTE.textPrimary, fontSize: 14, fontWeight: 600, fontFamily: "'DM Sans', sans-serif" }}>Bonne réponse !</div>
            <div style={{ color: PALETTE.textSecondary, fontSize: 12, fontFamily: "'DM Sans', sans-serif" }}>Continuez, plus que 3 mots</div>
          </div>
        </div>
      </div>
    );
  }

  // Method selection for a song
  if (selectedSong && !selectedMethod) {
    return (
      <div style={{ padding: "8px 20px 20px" }}>
        <div style={{ display: "flex", alignItems: "center", gap: 12, marginBottom: 8 }}>
          <button onClick={() => setSelectedSong(null)} style={{ background: "none", border: "none", color: PALETTE.textSecondary, fontSize: 20, cursor: "pointer", padding: 0 }}>‹</button>
          <h2 style={{ color: PALETTE.textPrimary, fontFamily: "'Cormorant Garamond', serif", fontSize: 22, margin: 0, fontWeight: 400 }}>
            Comment <span style={{ color: PALETTE.accentGold }}>mémoriser</span> ?
          </h2>
        </div>

        {/* Song card */}
        <div style={{
          background: PALETTE.bgCard, borderRadius: 16, padding: 16,
          display: "flex", gap: 14, alignItems: "center", marginBottom: 28, marginTop: 16,
          border: `1px solid ${PALETTE.border}`,
        }}>
          <div style={{
            width: 56, height: 56, borderRadius: 14,
            background: `linear-gradient(135deg, ${PALETTE.accentViolet}30, ${PALETTE.bgDeep})`,
            display: "flex", alignItems: "center", justifyContent: "center", fontSize: 28,
          }}>{selectedSong.cover}</div>
          <div>
            <div style={{ color: PALETTE.textPrimary, fontSize: 16, fontWeight: 600, fontFamily: "'DM Sans', sans-serif" }}>{selectedSong.title}</div>
            <div style={{ color: PALETTE.textSecondary, fontSize: 13, fontFamily: "'DM Sans', sans-serif" }}>{selectedSong.artist} · {langFlags[selectedSong.lang]}</div>
          </div>
        </div>

        {/* Methods grid */}
        <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 12 }}>
          {methods.map((m) => (
            <button key={m.id} onClick={() => setSelectedMethod(m.id)} style={{
              background: PALETTE.bgCard, borderRadius: 16, padding: 20,
              border: `1px solid ${PALETTE.border}`, cursor: "pointer", textAlign: "left",
            }}>
              <div style={{
                width: 40, height: 40, borderRadius: 12,
                background: `${m.color}20`,
                display: "flex", alignItems: "center", justifyContent: "center",
                fontSize: 18, color: m.color, marginBottom: 12,
                fontFamily: "monospace", fontWeight: 700,
              }}>{m.icon}</div>
              <div style={{ color: PALETTE.textPrimary, fontSize: 14, fontWeight: 600, fontFamily: "'DM Sans', sans-serif", marginBottom: 4 }}>{m.name}</div>
              <div style={{ color: PALETTE.textMuted, fontSize: 11, fontFamily: "'DM Sans', sans-serif", lineHeight: 1.4 }}>{m.desc}</div>
            </button>
          ))}
        </div>
      </div>
    );
  }

  // Song selection
  return (
    <div style={{ padding: "8px 20px 20px" }}>
      <div style={{ marginBottom: 20 }}>
        <h1 style={{ color: PALETTE.textPrimary, fontFamily: "'Cormorant Garamond', serif", fontSize: 28, margin: 0, fontWeight: 400 }}>
          <span style={{ color: PALETTE.accentGold }}>Mémoriser</span>
        </h1>
        <p style={{ color: PALETTE.textMuted, fontSize: 12, margin: "4px 0 0", fontFamily: "'DM Sans', sans-serif" }}>Choisissez une chanson à apprendre</p>
      </div>

      {/* Progress overview */}
      <div style={{
        background: `linear-gradient(135deg, ${PALETTE.accentGold}10, ${PALETTE.accentViolet}08)`,
        borderRadius: 16, padding: 18, marginBottom: 24,
        border: `1px solid ${PALETTE.accentGold}20`,
        display: "flex", justifyContent: "space-around",
      }}>
        {[
          { val: "3", label: "En cours" },
          { val: "48%", label: "Moyenne" },
          { val: "12", label: "Vers maîtrisés" },
        ].map((s) => (
          <div key={s.label} style={{ textAlign: "center" }}>
            <div style={{ color: PALETTE.accentGold, fontSize: 22, fontFamily: "'Cormorant Garamond', serif", fontWeight: 600 }}>{s.val}</div>
            <div style={{ color: PALETTE.textMuted, fontSize: 10, fontFamily: "'DM Sans', sans-serif" }}>{s.label}</div>
          </div>
        ))}
      </div>

      {/* Song list with progress */}
      {mockSongs.map((song) => (
        <button key={song.id} onClick={() => setSelectedSong(song)} style={{
          background: PALETTE.bgCard, borderRadius: 14, padding: 14,
          marginBottom: 10, display: "flex", gap: 14, alignItems: "center",
          border: `1px solid ${PALETTE.border}`, width: "100%", cursor: "pointer", textAlign: "left",
        }}>
          <div style={{
            width: 48, height: 48, borderRadius: 12,
            background: `linear-gradient(135deg, ${PALETTE.accentViolet}30, ${PALETTE.bgDeep})`,
            display: "flex", alignItems: "center", justifyContent: "center", fontSize: 22,
          }}>{song.cover}</div>
          <div style={{ flex: 1 }}>
            <div style={{ color: PALETTE.textPrimary, fontSize: 14, fontWeight: 600, fontFamily: "'DM Sans', sans-serif" }}>{song.title}</div>
            <div style={{ color: PALETTE.textSecondary, fontSize: 12, fontFamily: "'DM Sans', sans-serif" }}>{song.artist}</div>
          </div>
          <div style={{ textAlign: "right" }}>
            <div style={{ color: PALETTE.accentGold, fontSize: 16, fontWeight: 700, fontFamily: "'DM Sans', sans-serif" }}>{song.progress}%</div>
            <div style={{ color: PALETTE.textMuted, fontSize: 10, fontFamily: "'DM Sans', sans-serif" }}>mémorisé</div>
          </div>
        </button>
      ))}
    </div>
  );
}

// ─── MAIN APP ─────────────────────────────────────────
export default function LyrememberWireframes() {
  const [activeScreen, setActiveScreen] = useState("overview");
  const [phoneTab, setPhoneTab] = useState("lyre");
  const [showFab, setShowFab] = useState(false);

  return (
    <div style={{
      minHeight: "100vh",
      background: `radial-gradient(ellipse at top, #13102280, ${PALETTE.bgDeep})`,
      fontFamily: "'DM Sans', sans-serif",
      color: PALETTE.textPrimary,
    }}>
      <link href="https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,300;0,400;0,500;0,600;1,400&family=DM+Sans:wght@400;500;600;700&display=swap" rel="stylesheet" />

      {/* Top header */}
      <div style={{ display: "flex", alignItems: "center", justifyContent: "center", padding: "20px 32px 0", gap: 10 }}>
        <LyreSvg size={24} color={PALETTE.accentGold} />
        <span style={{ fontFamily: "'Cormorant Garamond', serif", fontSize: 22, color: PALETTE.textPrimary, fontWeight: 300 }}>
          Lyre<span style={{ color: PALETTE.accentGold }}>member</span>
        </span>
      </div>
      <p style={{ textAlign: "center", color: PALETTE.textMuted, fontSize: 13, margin: "4px 0 16px" }}>Wireframes v2 — Comprendre & Mémoriser</p>

      {/* Screen tabs */}
      <div style={{ display: "flex", justifyContent: "center", gap: 6, padding: "0 20px 28px", flexWrap: "wrap" }}>
        {[
          { id: "overview", label: "📐 Architecture" },
          { id: "lyre", label: "📖 Ma Lyre" },
          { id: "ajouter", label: "＋ Ajouter" },
          { id: "memoriser", label: "🧩 Mémoriser" },
        ].map((tab) => (
          <button key={tab.id} onClick={() => { setActiveScreen(tab.id); if (tab.id === "lyre") setPhoneTab("lyre"); if (tab.id === "memoriser") setPhoneTab("memorize"); }} style={{
            background: activeScreen === tab.id ? PALETTE.accentGold : PALETTE.bgCard,
            color: activeScreen === tab.id ? PALETTE.bgDeep : PALETTE.textSecondary,
            border: activeScreen === tab.id ? "none" : `1px solid ${PALETTE.border}`,
            borderRadius: 12, padding: "10px 18px", fontSize: 13,
            fontWeight: activeScreen === tab.id ? 600 : 400,
            cursor: "pointer", fontFamily: "'DM Sans', sans-serif",
          }}>{tab.label}</button>
        ))}
      </div>

      {/* Content */}
      <div style={{ display: "flex", justifyContent: "center", paddingBottom: 60 }}>

        {activeScreen === "overview" && (
          <div style={{ maxWidth: 700, padding: "0 24px", width: "100%" }}>
            {/* Architecture overview */}
            <div style={{ marginBottom: 40 }}>
              <p style={{ color: PALETTE.accentGold, fontFamily: "'Cormorant Garamond', serif", fontSize: 14, letterSpacing: 3, textTransform: "uppercase", marginBottom: 8 }}>Navigation</p>
              <h2 style={{ color: PALETTE.textPrimary, fontFamily: "'Cormorant Garamond', serif", fontSize: 32, fontWeight: 300, margin: 0 }}>
                2 onglets + 1 action
              </h2>
              <p style={{ color: PALETTE.textSecondary, fontSize: 14, marginTop: 10, lineHeight: 1.6 }}>
                Une navigation minimale centrée sur les deux axes de l'app : comprendre les paroles et les mémoriser.
              </p>
            </div>

            {/* Nav diagram */}
            <div style={{ display: "flex", gap: 16, marginBottom: 40, flexWrap: "wrap", justifyContent: "center" }}>
              {[
                { icon: <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke={PALETTE.accentGold} strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round"><path d="M2 4C4 3 7 3 9 4C11 5 12 5 12 5V21C12 21 11 20 9 19.5C7 19 4 19 2 20V4Z"/><path d="M22 4C20 3 17 3 15 4C13 5 12 5 12 5V21C12 21 13 20 15 19.5C17 19 20 19 22 20V4Z"/></svg>, name: "Ma Lyre", desc: "Collection + lecture bilingue des paroles", features: ["Collection filtrée par langue", "Recherche dans les titres", "Lecture paroles + traduction", "Sélecteur de langue cible", "Notes & contexte"], type: "tab" },
                { icon: <span style={{ fontSize: 28, color: PALETTE.bgDeep }}>+</span>, name: "Ajouter", desc: "Bouton central flottant (FAB)", features: ["Recherche via API musicale", "Saisie manuelle titre/artiste", "Import/saisie des paroles", "Choix de la langue originale", "Note personnelle"], type: "fab" },
                { icon: <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke={PALETTE.accentGold} strokeWidth="1.6" strokeLinecap="round" strokeLinejoin="round"><path d="M4 4H9C9 4 9 2 10.5 2C12 2 12 4 12 4H18V9C18 9 20 9 20 10.5C20 12 18 12 18 12V18H12C12 18 12 20 10.5 20C9 20 9 18 9 18H4V12C4 12 2 12 2 10.5C2 9 4 9 4 9V4Z"/></svg>, name: "Mémoriser", desc: "Exercices pour apprendre par cœur", features: ["Texte à trous", "Première lettre", "Karaoké (défilement)", "Flashcards vers par vers", "Progression par chanson"], type: "tab" },
              ].map((item) => (
                <div key={item.name} style={{
                  background: PALETTE.bgCard, borderRadius: 20, padding: 24, flex: "1 1 200px",
                  border: `1px solid ${item.type === "fab" ? PALETTE.accentGold + "40" : PALETTE.border}`,
                  maxWidth: 220,
                }}>
                  <div style={{
                    width: 52, height: 52, borderRadius: 14,
                    background: item.type === "fab" ? PALETTE.accentGold : `${PALETTE.accentViolet}20`,
                    display: "flex", alignItems: "center", justifyContent: "center",
                    marginBottom: 14,
                  }}>{item.icon}</div>
                  <div style={{ color: PALETTE.textPrimary, fontSize: 16, fontWeight: 600, fontFamily: "'DM Sans', sans-serif", marginBottom: 4 }}>{item.name}</div>
                  <div style={{ color: PALETTE.textMuted, fontSize: 12, fontFamily: "'DM Sans', sans-serif", marginBottom: 14, lineHeight: 1.4 }}>{item.desc}</div>
                  <div style={{ borderTop: `1px solid ${PALETTE.border}`, paddingTop: 12 }}>
                    {item.features.map((f, i) => (
                      <div key={i} style={{ display: "flex", alignItems: "center", gap: 8, marginBottom: 6 }}>
                        <span style={{ color: PALETTE.accentGold, fontSize: 8 }}>●</span>
                        <span style={{ color: PALETTE.textSecondary, fontSize: 12, fontFamily: "'DM Sans', sans-serif" }}>{f}</span>
                      </div>
                    ))}
                  </div>
                </div>
              ))}
            </div>

            {/* User flow */}
            <div style={{
              background: PALETTE.bgCard, borderRadius: 20, padding: 28,
              border: `1px solid ${PALETTE.border}`,
            }}>
              <p style={{ color: PALETTE.accentGold, fontSize: 11, letterSpacing: 2, textTransform: "uppercase", margin: "0 0 20px", fontFamily: "'DM Sans', sans-serif" }}>Parcours utilisateur</p>
              <div style={{ display: "flex", alignItems: "center", gap: 12, flexWrap: "wrap", justifyContent: "center" }}>
                {[
                  { step: "1", label: "Ajouter", desc: "une chanson + paroles" },
                  { step: "→" },
                  { step: "2", label: "Comprendre", desc: "lecture bilingue dans Ma Lyre" },
                  { step: "→" },
                  { step: "3", label: "Mémoriser", desc: "exercices interactifs" },
                ].map((s, i) => (
                  s.step === "→" ? (
                    <span key={i} style={{ color: PALETTE.accentGold, fontSize: 20 }}>→</span>
                  ) : (
                    <div key={i} style={{
                      background: PALETTE.surface, borderRadius: 14, padding: "14px 18px",
                      border: `1px solid ${PALETTE.border}`, textAlign: "center", flex: "1 1 120px",
                    }}>
                      <div style={{
                        width: 28, height: 28, borderRadius: 14,
                        background: `${PALETTE.accentGold}20`, color: PALETTE.accentGold,
                        display: "flex", alignItems: "center", justifyContent: "center",
                        fontSize: 13, fontWeight: 700, margin: "0 auto 8px", fontFamily: "'DM Sans', sans-serif",
                      }}>{s.step}</div>
                      <div style={{ color: PALETTE.textPrimary, fontSize: 14, fontWeight: 600, fontFamily: "'DM Sans', sans-serif" }}>{s.label}</div>
                      <div style={{ color: PALETTE.textMuted, fontSize: 11, fontFamily: "'DM Sans', sans-serif", marginTop: 4 }}>{s.desc}</div>
                    </div>
                  )
                ))}
              </div>
            </div>
          </div>
        )}

        {activeScreen === "lyre" && (
          <PhoneFrame title="Ma Lyre — Collection & paroles bilingues" activeTab="lyre" onTabChange={(t) => { setPhoneTab(t); setActiveScreen(t === "memorize" ? "memoriser" : "lyre"); }} onFabClick={() => setActiveScreen("ajouter")}>
            <MaLyreScreen />
          </PhoneFrame>
        )}

        {activeScreen === "ajouter" && (
          <PhoneFrame title="Ajouter — Bouton central (FAB)">
            <AjouterScreen onClose={() => setActiveScreen("lyre")} />
          </PhoneFrame>
        )}

        {activeScreen === "memoriser" && (
          <PhoneFrame title="Mémoriser — Exercices interactifs" activeTab="memorize" onTabChange={(t) => { setPhoneTab(t); setActiveScreen(t === "memorize" ? "memoriser" : "lyre"); }} onFabClick={() => setActiveScreen("ajouter")}>
            <MemoriserScreen />
          </PhoneFrame>
        )}
      </div>
    </div>
  );
}
