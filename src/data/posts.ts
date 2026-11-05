import raspberryPiSource from "@/pages/2025-07-15.md?raw";
import whatToEatSource from "@/pages/2024-11-18.md?raw";

export type Post = {
  slug: string;
  title: string;
  date: string;
  datetime: string;
  kicker: string;
  lead: string;
  source?: { label: string; href: string };
  body: string;
};

/** 按时间倒序。新文章加在数组最前面即可。 */
export const posts: Post[] = [
  {
    slug: "raspberry-pi-network",
    title: "有了树莓派以后，我是如何改造家里的网络的？",
    date: "2025.07.15",
    datetime: "2025-07-15",
    kicker: "折腾记录 / 家庭网络",
    lead: "从 mihomo 的 TUN 模式，到路由器、交换机和树莓派各自该待在哪里。这是一篇关于把全屋网络重新理顺的折腾记录。",
    body: raspberryPiSource,
  },
  {
    slug: "what-to-eat-today",
    title: "还在愁不知道今天吃什么？P 人福音来啦！自制 App 帮你解决选择困难症",
    date: "2024.11.18",
    datetime: "2024-11-18",
    kicker: "前端 / 小工具",
    lead: "从 Tkinter 到原生 HTML，再到 Ionic + Vue3。一个「今天吃什么」的小应用，被我重写了三遍。",
    source: {
      label: "公众号「一只会魔法的梨」",
      href: "https://mp.weixin.qq.com/s/HY8e9j7Ys0-VVo7Y5Ce5OQ",
    },
    body: whatToEatSource,
  },
];

export function findPost(slug: string): Post | undefined {
  return posts.find((post) => post.slug === slug);
}
