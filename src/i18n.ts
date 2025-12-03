import { getLocaleFromNavigator, init, register } from "svelte-i18n";

export const avaliableLocales = ["en", "zh-CN"];

export function initI18n(){
  for(const locale of avaliableLocales){
    register(locale, () => import(`./locales/${locale}.json`));
  }
  init({
    fallbackLocale: 'en',
    initialLocale: localStorage.getItem('locale') || getLocaleFromNavigator()
  })
}
