<script>
  import { currentProject, contents } from '../../stores.js';

  const createContents = () => {};
  const listProjects = () => {};
</script>
<section>
  <div class="legend">
    <h1 class="nowrap">
      {#if $currentProject} {$currentProject} {:else} No project {/if}
    </h1>
    <button class="create-button" on:click="{listProjects}">
      <img
        src="svg/file.svg"
        alt="プロジェクトリスト"
        title="プロジェクトリスト"
      />
    </button>
  </div>

  <nav class="frame legend">
    <h2 class="nowrap">CONTENTS</h2>
    <button class="create-button" on:click="{createContents}">+</button>
  </nav>

  <div class="contents">
    {#if $contents} {#each $contents as content}
    <button class="content-btn">
      <img src="svg/archive.svg" alt="" />
      <p>{content}</p>
    </button>
    {/each} {/if}
  </div>
</section>

<style>
  section {
    font-family: var(--mono);
    background-color: var(--cl-exp-bg);
    color: var(--cl-exp-txt);
    width: 250px;
    height: 100%;
    resize: horizontal;
  }

  h1 {
    color: var(--cl-exp-txt);
    padding: 0.4rem 1.2rem;
    font-size: 0.8rem;
    text-transform: uppercase;
    font-weight: 100;
  }

  .legend {
    width: 100%;
    padding: 0.6rem 1.2rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  h2 {
    font-size: 1.2rem;
  }

  .create-button {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    background-color: inherit;
    color: inherit;
  }

  .contents {
    width: 100%;
    padding-top: 0.6rem;
  }
  .content-btn {
    display: flex;
    color: inherit;
    background-color: inherit;
    padding: 1rem 1.4rem;
  }
  .content-btn > img {
    width: 18px;
    height: 18px;
  }
  .content-btn > p {
    white-space: nowrap;
    text-overflow: ellipsis;
    margin-left: 1rem;
  }
</style>
