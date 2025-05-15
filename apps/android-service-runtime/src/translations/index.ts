import { register, init, getLocaleFromNavigator } from 'svelte-i18n';

const ALL_LOCALES = ["bg", "da", "de", "en", "es", "fr", "it", "no", "ro", "sk", "sv", "nl"];

export function initTranslations() {
  ALL_LOCALES.forEach((locale) => register(locale, () => import(`./locales/${locale}.json`)));

  init({
    fallbackLocale: 'en',
    initialLocale: getLocaleFromNavigator(),
  });
}