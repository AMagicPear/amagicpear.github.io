<script lang="ts">
  import type { PerryWork } from "@/data/works";

  let cardTransform: string = $state<string>("rotateX(0) rotateY(0) translateZ(0)");
  let lightTransform: string = $state<string>("translate(0, 0)");
  let cardContainer: HTMLDivElement;
  let isHovered: boolean = $state(false);
  let props: PerryWork = $props();

  const prefersReducedMotion = window.matchMedia(
    "(prefers-reduced-motion: reduce)",
  ).matches;

  const getCenter = (element: HTMLElement) => {
    const rect = element.getBoundingClientRect();
    return {
      x: rect.left + rect.width / 2,
      y: rect.top + rect.height / 2,
    };
  };

  const handleMouseMove = (e: MouseEvent) => {
    const cardCenter = getCenter(cardContainer);
    const mouseX = e.clientX - cardCenter.x;
    const mouseY = e.clientY - cardCenter.y;
    const rotateY = (mouseX / cardContainer.offsetWidth) * 6;
    const rotateX = -(mouseY / cardContainer.offsetHeight) * 6;
    cardTransform = `rotateX(${rotateX}deg) rotateY(${rotateY}deg) translateZ(16px)`;
    const lightX = (mouseX / cardContainer.offsetWidth) * 100;
    const lightY = (mouseY / cardContainer.offsetHeight) * 100;
    lightTransform = `translate(${lightX}px, ${lightY}px)`;
  };

  const handleMouseEnter = () => {
    isHovered = true;
  };

  const handleMouseLeave = () => {
    isHovered = false;
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
  onmouseenter={!prefersReducedMotion ? handleMouseEnter : null}
  onmouseleave={!prefersReducedMotion ? handleMouseLeave : null}
>
  <div class="card" style:transform={cardTransform}>
    <div class="card-border-glow" class:active={isHovered}></div>

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
        style:background={`radial-gradient(circle at center, ${props.lightColor} 0%, transparent 70%)`}
      ></div>
    {/if}

    <div class="card-top-rule"></div>

    <div class="card-content">
      <div class="card-header">
        <div class="classify-tag">{props.classify}</div>
        <h2>{props.cardTitle}</h2>
        <p>{props.description}</p>
      </div>

      <div class="card-footer">
        <a href={props.link} target="_blank" class="dive-link">
          <span>Dive In</span>
          <span class="arrow">&rarr;</span>
        </a>
      </div>
    </div>

    <div class="card-corner-accent"></div>
  </div>
</div>

<style lang="scss">
  .card-container {
    width: 380px;
    max-width: calc(100vw - 40px);
    margin: 24px;
    perspective: 1200px;
  }

  .card {
    position: relative;
    width: 100%;
    min-height: 280px;
    transition: transform 0.2s ease-out;
    transform-style: preserve-3d;
    background: var(--color-surface);
    border: 1px solid var(--color-border);
    overflow: hidden;
    will-change: transform;

    .card-border-glow {
      position: absolute;
      inset: -1px;
      z-index: 0;
      pointer-events: none;
      opacity: 0;
      transition: opacity 0.4s ease;
      background: linear-gradient(
        135deg,
        rgba(200, 255, 54, 0.3),
        transparent 40%,
        transparent 60%,
        rgba(200, 255, 54, 0.1)
      );

      &.active {
        opacity: 1;
      }
    }

    .card-top-rule {
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      height: 2px;
      background: linear-gradient(
        90deg,
        transparent,
        var(--color-chartreuse-dim) 20%,
        var(--color-chartreuse-dim) 80%,
        transparent
      );
      z-index: 5;
      opacity: 0;
      transition: opacity 0.4s ease;
    }

    &:hover .card-top-rule {
      opacity: 1;
    }

    .card-corner-accent {
      position: absolute;
      bottom: 16px;
      right: 16px;
      width: 32px;
      height: 32px;
      border-right: 1px solid rgba(200, 255, 54, 0.2);
      border-bottom: 1px solid rgba(200, 255, 54, 0.2);
      z-index: 5;
      pointer-events: none;
      transition: all 0.4s ease;
    }

    &:hover .card-corner-accent {
      border-color: rgba(200, 255, 54, 0.5);
      width: 40px;
      height: 40px;
    }
  }

  picture {
    position: absolute;
    inset: 0;
    opacity: 0.06;
    transition: opacity 0.4s ease;

    img {
      width: 100%;
      height: 100%;
      object-fit: cover;
      object-position: center;
    }
  }

  .card:hover picture {
    opacity: 0.1;
  }

  .card-content {
    position: relative;
    inset: 0;
    padding: 36px 32px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    min-height: 280px;
    transform: translateZ(16px);
    z-index: 10;
  }

  .card-header {
    text-align: left;

    .classify-tag {
      display: inline-block;
      font-family: var(--font-display);
      font-size: 10px;
      text-transform: uppercase;
      letter-spacing: 0.25em;
      color: var(--color-chartreuse);
      border: 1px solid rgba(200, 255, 54, 0.3);
      padding: 4px 12px;
      margin-bottom: 20px;
      opacity: 0.8;
    }

    h2 {
      font-family: var(--font-display);
      font-size: 26px;
      font-weight: 700;
      color: var(--color-text);
      margin: 0 0 14px 0;
      line-height: 1.2;
    }

    p {
      font-family: var(--font-body);
      font-size: 15px;
      line-height: 1.7;
      color: var(--color-text-muted);
    }
  }

  .card-footer {
    margin-top: 28px;
    text-align: left;
  }

  .dive-link {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    font-family: var(--font-display);
    font-size: 14px;
    font-weight: 600;
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.15em;
    transition: all 0.35s cubic-bezier(0.22, 0.61, 0.36, 1);

    .arrow {
      display: inline-block;
      transition: transform 0.35s cubic-bezier(0.22, 0.61, 0.36, 1);
    }

    &:hover {
      color: var(--color-chartreuse);
      gap: 14px;

      .arrow {
        transform: translateX(4px);
      }
    }
  }

  .light-effect {
    position: absolute;
    width: 240px;
    height: 240px;
    border-radius: 50%;
    z-index: 1;
    pointer-events: none;
    transition: transform 0.25s ease-out;
    opacity: 0.5;
  }
</style>
