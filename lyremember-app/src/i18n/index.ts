import { createI18n } from 'vue-i18n';
import en from './locales/en.json';
import fr from './locales/fr.json';
import ja from './locales/ja.json';
import ko from './locales/ko.json';

export type SupportedLocale = 'en' | 'fr' | 'ja' | 'ko';

export const supportedLocales: { code: SupportedLocale; label: string }[] = [
  { code: 'en', label: 'English' },
  { code: 'fr', label: 'Français' },
  { code: 'ja', label: '日本語' },
  { code: 'ko', label: '한국어' },
];

function getInitialLocale(): SupportedLocale {
  const saved = localStorage.getItem('locale');
  if (saved && supportedLocales.some(l => l.code === saved)) {
    return saved as SupportedLocale;
  }
  const browserLang = navigator.language.split('-')[0];
  if (supportedLocales.some(l => l.code === browserLang)) {
    return browserLang as SupportedLocale;
  }
  return 'en';
}

const i18n = createI18n({
  legacy: false,
  locale: getInitialLocale(),
  fallbackLocale: 'en',
  messages: { en, fr, ja, ko },
});

export default i18n;
