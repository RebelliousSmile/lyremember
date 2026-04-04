import { createI18n } from 'vue-i18n';
import en from './locales/en.json';
import fr from './locales/fr.json';

export type SupportedLocale = 'en' | 'fr';

export const supportedLocales: { code: SupportedLocale; label: string }[] = [
  { code: 'en', label: 'English' },
  { code: 'fr', label: 'Français' },
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
  messages: { en, fr },
});

export default i18n;
