<script lang="ts">
  import PerryHeader from "./components/PerryHeader.svelte";
  import LeftCard from "./components/LeftCard.svelte";
  import PerryWaves from "./components/PerryWaves.svelte";
  import EmailIcon from "./assets/icons/email.svg";
  import GithubIcon from "./assets/icons/github.svg";
  import BilibiliIcon from "./assets/icons/bilibili.svg";
  import NeteaseMusicIcon from "./assets/icons/netease-music.svg";

  const contact = [
    {
      name: "Email",
      icon: EmailIcon,
      link: "mailto:AMagicPear@outlook.com",
    },
    {
      name: "Github",
      icon: GithubIcon,
      link: "https://github.com/AMagicPear",
    },
    {
      name: "Bilibili",
      icon: BilibiliIcon,
      link: "https://space.bilibili.com/52833994",
    },
    {
      name: "Netease Music",
      icon: NeteaseMusicIcon,
      link: "https://music.163.com/#/artist?id=34318509",
    },
  ];
</script>

<div class="background"></div>
<main>
  <section id="top-showcase">
    <PerryHeader />
    <div class="background-element">
      <PerryWaves />
    </div>
    <div class="floating-element">
      <LeftCard cardTitle="你可以叫我"></LeftCard>
      <LeftCard cardTitle="我的平台">
        <div class="contact-list">
          {#each contact as item}
            <a href={item.link} target="_blank">
              <img src={item.icon} alt={item.name} />
            </a>
          {/each}
        </div>
      </LeftCard>
    </div>
  </section>
  <section id="sub-content">
    <h2>Sub Content</h2>
  </section>
</main>

<footer>
  <p>AMagicPear &copy; 2025</p>
</footer>

<style lang="scss">
  #top-showcase {
    position: relative;
    width: 100%;
    height: 100vh;
    /* background-color: #212420; */
    /* 改成渐变 */
    background: linear-gradient(160deg, #242320 0%, #181a18 100%);
  }

  .background-element {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .floating-element {
    position: absolute;
    top: 55%;
    left: 30%;
    transform: translate(-50%, -50%);
    z-index: 3;
    .contact-list {
      display: flex;
      flex-direction: row;
      justify-content: center;
      align-items: flex-start;
      gap: 12px;
      img {
        max-width: 30px;
        max-height: 30px;
        margin-right: 10px;
        transition: transform 0.2s ease-in-out;

        &:hover {
          transform: scale(1.1);
        }
      }
    }
    :global(.left-card:nth-child(1)) {
      transform: translate(-50%, -50%) rotate(10deg);
    }
  }

  .background {
    pointer-events: none;
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100vh;
    background-size: 128px;
    background-repeat: repeat;
    background-image: url("./assets/rR6HYXBrMmX4cRpXfXUOvpvpB0.png");
    opacity: 0.06;
    border-radius: 0;
    z-index: 400;
  }

  footer {
    position: fixed;
    left: 2vw;
    bottom: 2px;
    z-index: 50;
    p {
      color: rgb(210, 246, 210);
      font-size: 14px;
      line-height: 1rem;
    }
  }
</style>
