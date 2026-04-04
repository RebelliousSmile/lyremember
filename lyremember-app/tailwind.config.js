/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        gold: {
          DEFAULT: '#F2A93B',
          light: '#F5C06A',
          dark: '#D4911F',
        },
        deep: {
          DEFAULT: '#0D0B1A',
          card: '#1A1528',
          'card-hover': '#231D35',
          surface: '#131022',
          border: '#2A2440',
        },
        violet: {
          accent: '#7B6FA0',
          muted: '#5E5480',
        },
      },
    },
  },
  plugins: [],
}
