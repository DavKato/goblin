<script>
  import EditorMode from './components/EditorMode.svelte'
  import ExplorerMode from './components/ExplorerMode.svelte'

  let mode = ExplorerMode
  const toggleModes = () => {
    mode = mode === ExplorerMode ? EditorMode : ExplorerMode
  }
</script>

<nav class="frame">
  <button on:click="{toggleModes}">toggle mode</button>
</nav>
<svelte:component this="{mode}"></svelte:component>

<style>
  :global(body) {
    /* FONTS */
    --mono: 'Century Gothic', Arial, monospace;
    --sans: 'Hiragano Sans', sans-serif;
    --serif: serif;

    /* BACKGROUND COLOR */
    --cl-frame-bg: #333;
    --cl-exp-bg: #252526;
    --cl-edt-bg: #1e1e1e;

    /* TEXT COLOR */
    --cl-exp-txt: #cacaca;
    --cl-frame-txt: #ccc;
  }

  :global(.frame) {
    background-color: var(--cl-frame-bg);
    color: var(--cl-frame-txt);
  }
  nav {
    width: 100%;
    text-align: right;
    border-bottom: 1px solid #000;
  }
</style>
