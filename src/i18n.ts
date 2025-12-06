import { getLocaleFromNavigator, init, register } from "svelte-i18n";

export enum Language {
  EN = "en",
  ZH_CN = "zh-CN",
}

export const avaliableLocales = [Language.EN, Language.ZH_CN];

export function initI18n() {
  for (const locale of avaliableLocales) {
    register(locale, () => import(`./locales/${locale}.json`));
  }
  init({
    fallbackLocale: Language.EN,
    initialLocale: localStorage.getItem("locale") || getLocaleFromNavigator(),
  });
}
