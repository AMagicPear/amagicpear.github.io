<script lang="ts">
  const contact = [
    {
      name: "Email",
      icon: "email",
      link: "mailto:AMagicPear@outlook.com",
    },
    {
      name: "Github",
      icon: "github",
      link: "https://github.com/AMagicPear",
    },
    {
      name: "Bilibili",
      icon: "bilibili",
      link: "https://space.bilibili.com/52833994",
    },
    {
      name: "Netease Music",
      icon: "netease-music",
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
        <img src={`/icons/${item.icon}.svg`} alt={item.name} />
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
    z-index: -1;
    border: 1px solid rgb(160, 255, 36);
    pointer-events: none;
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
