<script>
  import { createProject } from '../../helpers/project.js';
  import { newProject } from '../../stores.js';
  import Modal from '../Modal/Modal.svelte';
  import Button from '../Unit/Button.svelte';
  import X from '../Unit/X.svelte';

  let projectName = '';
  $: if ($newProject) projectName = '';
</script>

{#if $newProject}
<Modal>
  <X
    style="position: absolute; top: -15px; right: -20px;"
    on:click="{() => $newProject = false}"
  ></X>
  <form on:submit|preventDefault="{() => createProject(projectName)}">
    <label for="project-name">プロジェクト名: </label>
    <input
      id="project-name"
      bind:value="{projectName}"
      type="text"
      minlength="2"
      maxlength="18"
      autofocus
      required
      pattern="[A-Za-z0~9!\ ]+"
    />
    <Button type="submit" color="blue">新しいスタートだ！</Button>
  </form>
</Modal>
{/if}

<style>
  form {
    display: flex;
    flex-direction: column;
    align-items: center;
    font-size: 1.2rem;
  }
  input {
    margin: 15px 0 30px;
    font-size: 1.2em;
    font-family: var(--mono);
    padding: 12px 30px;
    width: 300px;
    border: 1px solid var(--cl-frame-bg);
    border-radius: 10px;
    outline: none;
    text-align: center;
  }
</style>
