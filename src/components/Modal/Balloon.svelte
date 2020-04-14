<script>
  import { newProject } from '../../stores.js';
  import Button from '../Unit/Button.svelte';
</script>

<div class="container">
  <div class="balloon">
    <p>最初のプロジェクトを作るごぶ！</p>
    <Button
      color="blue"
      css="
        position: absolute;
        top: 65%;
        left: 50%;
        transform: translate(-50%, -50%);
      "
      on:click="{() => $newProject = true}"
    >
      プロジェクト作成
    </Button>
  </div>
  <div class="img-box">
    <img src="img/goblin.png" alt="" />
  </div>
</div>

<style>
  .container {
    margin: 0 auto;
    align-self: center;
    background-color: transparent;
  }
  .balloon {
    position: relative;
    width: 280px;
    height: 150px;
    border-radius: 50%;
    background-color: var(--cl-stdout-txt);
  }
  .balloon > p {
    position: absolute;
    top: 40%;
    left: 50%;
    transform: translate(-50%, -50%);
    white-space: nowrap;
    color: var(--cl-edt-bg);
    font-weight: 700;
    font-family: var(--sans);
  }
  .balloon:after {
    content: '';
    position: absolute;
    bottom: -25px;
    right: 50px;
    border-top: 50px solid var(--cl-stdout-txt);
    border-right: 35px solid transparent;
    border-left: 4px solid transparent;
  }

  .img-box {
    margin-top: 20px;
    width: 200px;
    height: 200px;
  }
  img {
    width: 100%;
    height: 100%;
    filter: drop-shadow(-2px 4px 5px var(--cl-sh));
  }
</style>
