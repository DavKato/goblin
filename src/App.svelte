<script>
  import { getError, query } from './helpers/messenger.js';
  import MessageBoard from './components/Modal/MessageBoard.svelte';
  import NewProject from './components/Modal/NewProject.svelte';
  import EditorMode from './components/EditorMode.svelte';
  import ExplorerMode from './components/ExplorerMode.svelte';

  let mode = ExplorerMode;
  const toggleModes = () => {
    mode = mode === ExplorerMode ? EditorMode : ExplorerMode;
  };
</script>

<nav class="frame">
  <button on:click="{getError}">send error</button>
  <button on:click="{query}">query</button>
  <button on:click="{toggleModes}">toggle mode</button>
</nav>

<MessageBoard></MessageBoard>

<NewProject></NewProject>

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
    --cl-frame-txt: #ccc;
    --cl-exp-txt: #cacaca;
    --cl-stdout-txt: #fcfcfc;

    /* SHADOW COLOR */
    --cl-sh: rgba(204, 204, 204, 0.3);
  }

  :global(.frame) {
    background-color: var(--cl-frame-bg);
    color: var(--cl-frame-txt);
  }
  :global(.nowrap) {
    white-space: nowrap;
    text-overflow: ellipsis;
  }
  :global(.center) {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }
  :global(.modal) {
    background-color: var(--cl-stdout-txt);
    color: var(--cl-edt-bg);
    border: 10px solid var(--cl-frame-txt);
  }
  :global(.shadow) {
    box-shadow: 2px 3px 4px rgba(0, 0, 0, 0.25);
  }
  :global(.shadow:hover) {
    box-shadow: 1px 2px 5px rgba(0, 0, 0, 0.25);
  }
  nav {
    width: 100%;
    text-align: right;
    border-bottom: 1px solid #000;
  }
</style>
