<template>
  <section class="profile">
    <div class="profile__container">
      <h2 class="profile__title" data-aos="zoom-out-up">关于我</h2>
      
      <div class="profile__content">
        <div class="profile__image" data-aos="fade-right">
          <img src="/images/me.webp" alt="个人照片">
        </div>
        
        <div class="profile__skills">
          <div 
            v-for="item in abilities" 
            :key="item.title"
            class="skill-card"
            :style="{ borderTop: `0.3rem solid ${item.color}` }"
            data-aos="zoom-out-up"
            data-aos-easing="ease-in-out"
          >
            <div class="skill-card__icon">
              <img :src="`/icons/${item.icon}.svg`" :alt="item.title">
            </div>
            <span 
              class="skill-card__percentage"
              :style="{ color: item.color }"
            >
              {{ item.bfb }}
            </span>
            <h3 class="skill-card__title">{{ item.title }}</h3>
            <p class="skill-card__subtitle">{{ item.subTitle }}</p>
          </div>
        </div>
      </div>

      <div class="profile__more">
        <button class="profile__button" @click="scrollGoTo('perry3')">
          继续浏览
        </button>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { scrollGoTo } from '@/utils';
import abilities from '@/assets/data/abilities.json';
</script>

<style scoped>
.profile {
  width: 100%;
  padding: 5rem 1rem;
}

.profile__container {
  max-width: 75rem;
  margin: 0 auto;
}

.profile__title {
  /* 统一标题样式 */
}

.profile__content {
  display: flex;
  gap: 4rem;
  align-items: flex-start;
  justify-content: center;
  margin-bottom: 3rem;
}

.profile__image {
  flex-shrink: 0;
  width: 18.75rem;
  height: 18.75rem;
  border: 0.625rem solid #fff;
  border-radius: 50%;
  overflow: hidden;
  box-shadow: 0 0.8rem 1.6rem rgba(0, 0, 0, 0.1);
}

.profile__image img {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.profile__skills {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(15.625rem, 1fr));
  gap: 1.5rem;
  width: 100%;
}

.skill-card {
  position: relative;
  padding: 1.5rem;
  background: #fff;
  border-radius: 0.75rem;
  box-shadow: 0 0.5rem 1rem rgba(0, 0, 0, 0.08);
  transition: all 0.3s ease;
}

.skill-card:hover {
  transform: translateY(-0.3rem);
  box-shadow: 0 1rem 2rem rgba(0, 0, 0, 0.12);
}

.skill-card__icon {
  position: absolute;
  right: 1rem;
  bottom: 1rem;
  opacity: 0.1;
}

.skill-card__icon img {
  width: 3rem;
  height: 3rem;
}

.skill-card__percentage {
  font-size: 1.5rem;
  font-weight: 600;
  display: block;
  margin-bottom: 0.5rem;
}

.skill-card__title {
  font-size: 1.125rem;
  margin: 0 0 0.5rem;
  font-weight: 500;
}

.skill-card__subtitle {
  font-size: 0.875rem;
  color: #666;
  margin: 0;
}

.profile__more {
  text-align: center;
}

.profile__button {
  padding: 0.75rem 2rem;
  font-size: 1rem;
  background: transparent;
  border: 0.125rem solid currentColor;
  border-radius: 2rem;
  cursor: pointer;
  transition: all 0.3s ease;
}

.profile__button:hover {
  background: rgba(0, 0, 0, 0.05);
}

@media (max-width: 64em) {
  .profile__content {
    gap: 2rem;
  }
}

@media (max-width: 48em) {
  .profile__content {
    flex-direction: column;
    align-items: center;
  }
  
  .profile__image {
    width: 15.625rem;
    height: 15.625rem;
  }
}

@media (max-width: 30em) {
  .profile {
    padding: 3rem 1rem;
  }
  
  .profile__image {
    width: 12.5rem;
    height: 12.5rem;
  }
  
  .profile__skills {
    grid-template-columns: 1fr;
  }
}
</style>
