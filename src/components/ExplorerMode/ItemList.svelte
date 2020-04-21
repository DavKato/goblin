<script>
  import { CONFIG } from '../../stores.js';
  import Balloon from '../Modal/Balloon.svelte';

  export let itemList = [''];
</script>
<section>
  {#if $CONFIG.currentProject} ITEM LIST {:else}
  <Balloon on:click></Balloon>
  {/if}
</section>

<style>
  section {
    background-color: var(--cl-edt-bg);
    width: 100%;
    color: var(--cl-stdout-txt);
    display: flex;
  }
</style>
