<script lang="ts">
  let works = [0, 1, 2, 3];
</script>

<h2 data-aos="fade-up">我的作品</h2>
<div class="works-container">
  {#each works as work}
    <div class="work-item" data-aos="flip-up"></div>
  {/each}
</div>

<style lang="scss">
  .works-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
  }
  .work-item {
    width: 300px;
    height: 300px;
    margin: 20px;
    background-color: #fff;
    box-shadow: 0px 0px 10px rgba(0, 0, 0, 0.1);
  }
</style>
