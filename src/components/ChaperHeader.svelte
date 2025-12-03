<script lang="ts">
  export let title: string;
</script>

<div data-aos="fade-up">
  <h2>{title}</h2>
</div>

<style lang="scss">
  div {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-top: 16px;

    h2 {
      width: fit-content;
      height: fit-content;
      background-color: var(--color-light);
      color: var(--color-dark);
      padding: 0.2rem 0.8rem;
      transition: all 0.4s ease-out;
      border: solid 2px var(--color-dark);

      &:hover {
        border: solid 2px var(--color-dark);
        background-color: var(--color-dark);
        color: var(--color-light);
      }
    }
  }
</style>
