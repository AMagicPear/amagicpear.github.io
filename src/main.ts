import { mount } from "svelte";
import "./app.scss";
import App from "./App.svelte";
import AOS from "aos";
import "aos/dist/aos.css";
import { initI18n } from "./i18n";
import { waitLocale } from "svelte-i18n";

// localStorage.setItem('locale', 'en')
// localStorage.removeItem('locale')
// 插件初始化
initI18n();
await waitLocale();
AOS.init();

const app = mount(App, {
  target: document.getElementById("app")!,
});

export default app;
