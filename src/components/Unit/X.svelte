<script>
  export let style = '';
</script>
<div {style} class="modal" on:click>
  <svg viewBox="0 0 32 32">
    <g>
      <line x1="2" x2="30" y1="2" y2="30" />
      <line x1="2" x2="30" y1="30" y2="2" />
    </g>
  </svg>
</div>

<style>
  div {
    border-radius: 50%;
  }
  svg {
    display: block;
    width: 36px;
    height: 36px;
    cursor: pointer;
    transition: transform 0.2s;
  }
  svg:hover {
    transform: scale(1.05);
  }
  g {
    stroke: #ac0b0b;
    stroke-width: 3;
  }
</style>
