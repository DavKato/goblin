<script>
  import { fly } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';

  export let style = '';
</script>
<div
  class="modal center"
  {style}
  transition:fly="{{x: -80, duration: 600, opacity: 0, easing: quintOut}}"
>
  <slot></slot>
</div>

<style>
  div {
    padding: 40px 40px;
    border-radius: 10px;
    text-align: center;
    line-height: 1.7;
    z-index: 100;
  }
</style>
