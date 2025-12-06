<script lang="ts">
  import type { PerryWork } from "@/data/works";

  let cardTransform: string = $state<string>(
    "rotateX(0) rotateY(0) translateZ(0)",
  );
  let lightTransform: string = $state<string>("translate(0, 0)");
  let cardContainer: HTMLDivElement;
  let props: PerryWork = $props();

  const prefersReducedMotion = window.matchMedia(
    "(prefers-reduced-motion: reduce)",
  ).matches;

  // 获取卡片中心点
  const getCenter = (element: HTMLElement) => {
    const rect = element.getBoundingClientRect();
    return {
      x: rect.left + rect.width / 2,
      y: rect.top + rect.height / 2,
    };
  };
  // 处理鼠标移动
  const handleMouseMove = (e: MouseEvent) => {
    const cardCenter = getCenter(cardContainer);
    const mouseX = e.clientX - cardCenter.x;
    const mouseY = e.clientY - cardCenter.y;
    // 计算旋转角度
    const rotateY = (mouseX / cardContainer.offsetWidth) * 5; // 最大旋转5度
    const rotateX = -(mouseY / cardContainer.offsetHeight) * 5;
    // 应用3D变换
    cardTransform = `rotateX(${rotateX}deg) rotateY(${rotateY}deg) translateZ(10px)`;

    // 移动光效
    const lightX = (mouseX / cardContainer.offsetWidth) * 100;
    const lightY = (mouseY / cardContainer.offsetHeight) * 100;
    lightTransform = `translate(${lightX}px, ${lightY}px)`;
  };

  // 处理鼠标离开
  const handleMouseLeave = () => {
    cardTransform = "rotateX(0) rotateY(0) translateZ(0)";
    lightTransform = "translate(0, 0)";
  };
</script>

<div
  class="card-container"
  role="listitem"
  data-aos="flip-up"
  bind:this={cardContainer}
  onmousemove={!prefersReducedMotion ? handleMouseMove : null}
  onmouseleave={!prefersReducedMotion ? handleMouseLeave : null}
>
  <div class="card" style:transform={cardTransform}>
    {#if props.backgroundImg}
      <picture>
        <source srcset={props.backgroundImg} />
        <img alt="Cover" />
      </picture>
    {/if}
    {#if props.lightColor}
      <div
        class="light-effect"
        style:transform={lightTransform}
        style:background={`radial-gradient(circle, ${props.lightColor} 0%, rgba(0, 0, 0, 0) 70%)`}
      ></div>
    {/if}
    <div class="card-content">
      <div class="card-header">
        <h2>{props.cardTitle}</h2>
        <p>{props.description}</p>
      </div>

      <div class="card-footer">
        <div class="classify">{props.classify}</div>
        <a href={props.link} target="_blank"><button>Dive In</button></a>
      </div>
    </div>
  </div>
</div>

<style lang="scss">
  .card-container {
    width: 370px;
    max-width: 100%;
    /* height: 300px; */
    margin: 20px;
    perspective: 1200px;
  }

  .card {
    position: relative;
    width: 100%;
    height: 100%;
    transition: transform 0.15s ease-out;
    transform-style: preserve-3d;
    background-color: rgb(255, 255, 255, 0.5);
    box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.1);
    overflow: hidden;
    will-change: transform;
    @media (prefers-color-scheme: dark) {
      background-color: rgb(255, 255, 255, 0.1);
    }
  }

  picture {
    position: absolute;
    inset: 0;
    opacity: 0.1;

    img {
      width: 100%;
      height: 100%;
      object-fit: cover;
      object-position: center;
    }
  }

  .card-content {
    position: relative;
    inset: 0;
    padding: 30px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    transform: translateZ(10px);
    z-index: 10;
    backdrop-filter: blur(2px);
  }

  .card-header {
    text-align: left;
    h2 {
      font-size: 24px;
      color: var(--color-text);
    }
    p {
      line-height: 1.6;
      color: rgba(30, 24, 0, 0.6);
      @media (prefers-color-scheme: dark) {
        color: rgba(255, 250, 240, 0.6);
      }
    }
  }

  .card-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 20px;

    .classify {
      font-size: 18px;
      color: var(--color-text);
    }
  }

  button {
    background: linear-gradient(90deg, #111 0%, #bbb 100%);
    color: white;
    border: none;
    padding: 12px 28px;
    font-size: 16px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    &:hover {
      translate: 0 -3px;
      opacity: 0.9;
    }
    &:active {
      translate: 0 -1px;
      opacity: 0.8;
    }
  }

  .light-effect {
    position: absolute;
    width: 200px;
    height: 200px;
    border-radius: 50%;
    z-index: 1;
    pointer-events: none;
    transition: transform 0.2s ease-out;
  }
</style>
