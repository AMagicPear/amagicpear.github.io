<template>
  <div class="perry">
    <div class="topContent">
      <div>
        <div id="MainTitle">
          <span>你好，我是<br /></span>
          <span class="magicTyping"></span>
        </div>
        <div class="SubTitle">天有云兮云又痴.</div>
        <ul class="ContentContact">
          <li v-for="(item, index) in contact" :key="index">
            <a :href="item.link" target="_blank">
              <img :src="`/icons/${item.icon}.svg`" :alt="item.name" class="icon">
            </a>
          </li>
        </ul>
        <div class="magicLoadMore">
          <div class="loadMore" @click="scrollGoTo('perry2')">
            了解更多
          </div>
        </div>
      </div>
    </div>
    <div class="perryBg topBg"></div>
  </div>
</template>

<script setup lang="ts">
import { scrollGoTo } from '@/utils';
import Typed from 'typed.js';
import { onMounted } from 'vue';

const me = [
  "一只会魔法的梨",
  "AMagicPear", 
  "独立音乐人",
  "一个数字媒体技术的大学生",
  "INFP小蝴蝶"
];

const contact = [
  {
    name: "Email",
    icon: "email",
    link: "mailto:hello@amagicpear.sbs"
  },
  {
    name: "Github",
    icon: "github",
    link: "https://github.com/AMagicPear"
  },
  {
    name: "Bilibili",
    icon: "bilibili",
    link: "https://space.bilibili.com/52833994"
  },
  {
    name: "Netease Music",
    icon: "netease-music",
    link: "https://music.163.com/#/artist?id=34318509"
  }
];

const startTyping = () => {
  new Typed(".magicTyping", {
    strings: me,
    typeSpeed: 30,
    backDelay: 2000,
    backSpeed: 50,
    loop: true
  });
};

onMounted(() => {
  startTyping();
});
</script>

<style scoped>
.perry {
  height: 800px;
  overflow: hidden;
}

.topContent {
  position: absolute;
  left: 0;
  width: 50%;
  height: 100%;
  padding: 80px;
  display: flex;
  align-items: center;
  box-sizing: border-box;
  box-shadow: 10px 0 30px rgb(239, 223, 223);
  z-index: 2;
}

.topBg {
  background-image: url("/images/bg.webp");
}

.SubTitle {
  font-size: var(--contextSize);
  margin-bottom: 20px;
}

.ContentContact {
  display: flex;
  margin-bottom: 20px;
}

.ContentContact li {
  margin-right: 20px;
}

.icon {
  width: 26px;
}

@media (max-width: 930px) {
  .topContent {
    padding: 40px;
  }
}

@media (max-width: 560px) {
  .perry {
    height: 700px;
  }

  .topContent {
    width: 100%;
    height: 60%;
    bottom: 0;
  }

  .topBg {
    width: 100%;
    height: 40%;
    top: 0;
  }
}
</style>
