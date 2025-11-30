<script lang="ts">
  import EmailIcon from "../assets/icons/email.svg";
  import GithubIcon from "../assets/icons/github.svg";
  import BilibiliIcon from "../assets/icons/bilibili.svg";
  import NeteaseMusicIcon from "../assets/icons/netease-music.svg";

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


<div class="left-card">
  <div class="left-card-offset"></div>
  <h2>我的平台</h2>
  <div class="contact-list">
    {#each contact as item}
      <a href={item.link} target="_blank">
        <img src={item.icon} alt={item.name} />
      </a>
    {/each}
  </div>
</div>

<style lang="scss">
  .left-card-offset {
    position: absolute;
    top: -16px;
    left: 16px;
    width: 100%;
    height: 100%;
    border: 1px solid rgb(160, 255, 36);
    pointer-events: none;
    transition: border-color 0.3s ease-in-out;
  }

  .left-card {
    width: calc(240px + 6vw + 2vh);
    background-color: rgb(160, 255, 36);
    color: black;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: flex-start;
    padding-left: 20px;
    padding-bottom: 20px;
    transition:
      background-color 0.3s ease-in-out,
      translate 0.3s ease-in-out;

    &:hover {
      background-color: rgb(184, 255, 91);
      translate: 0px -2px;
      .left-card-offset {
        border-color: rgb(184, 255, 91);
      }
    }
  }

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
</style>
