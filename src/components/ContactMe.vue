<template>
  <div class="perry perry4">
    <!-- 壁纸 -->
    <div class="perryBg perry4Bg"></div>
    <!-- 内容 -->
    <div class="perryContent perry4Content">
      <div>
        <p class="mainContentTitle" data-aos="zoom-out-up">{{ contact.title }}</p>
        <ul>
          <li v-for="item in contact.list" :key="item.name">
            <p class="perry4ItemTitle">{{ item.name }}</p>
            <p class="perry4ItemContext">{{ item.context }}</p>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import contact from '@/assets/data/contact.json';
</script>

<style scoped>
.perry4 {
  height: 800px;
  position: relative;
  overflow: hidden;
}

.perry4Bg {
  background-image: url("/images/bg.webp");
}

.perry4Content {
  position: absolute;
  right: 0;
  width: 50%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 80px;
  box-sizing: border-box;
  box-shadow: -10px 0 30px rgb(239, 223, 223);
  z-index: 2;
}

.mainContentTitle {
  font-size: 2rem;
}

@media (max-width: 560px) {
  .perry4 {
    height: 700px;
  }

  .perry4Content {
    width: 100%;
    height: 60%;
    bottom: 0;
  }

  .perry4Bg {
    width: 100%;
    height: 40%;
    top: 0;
  }
}
</style>
