<script>
  import { scale } from 'svelte/transition';
  export let toggled = false;
</script>

<svg viewBox="0 0 24 20">
  {#if toggled}
  <line
    transition:scale="{{opacity: 1, duration: 150}}"
    x1="4"
    y1="0"
    x2="20"
    y2="20"
  ></line>
  <line
    transition:scale="{{opacity: 1, duration: 150}}"
    x1="20"
    y1="0"
    x2="4"
    y2="20"
  ></line>
  {:else}
  <polygon
    transition:scale="{{opacity: 1, duration: 150}}"
    points="0 0, 24 0, 12 20"
    fill="#fff"
  ></polygon>
  {/if}
</svg>

<style>
  svg {
    filter: drop-shadow(5px 5px 6px var(--d-shadow));
    margin-left: auto;
    margin-right: 25px;
    width: 25px;
    overflow: visible;
  }
  line {
    stroke: #fff;
    stroke-width: 3px;
    fill: none;
  }
  line,
  polygon {
    transform-origin: center;
  }
</style>
