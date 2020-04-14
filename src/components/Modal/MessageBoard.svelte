<script>
  import { error, success } from '../../stores.js';
  import Modal from '../Modal/Modal.svelte';
  import Button from '../Unit/Button.svelte';

  const ignore = () => {
    $error = '';
  };

  $: if ($success) setTimeout(() => ($success = ''), 2000);
</script>

{#if $error}
<Modal>
  <div class="error">
    <p>
      エラーが発生しました。
    </p>
    <p>
      開発者を怒鳴りつけてください。
    </p>
  </div>
  <fieldset>
    <legend>エラー内容</legend>
    <div class="message">
      {$error}
    </div>
  </fieldset>

  <div class="btns">
    <Button color="red">
      おい！（報告する）
    </Button>
    <Button css="margin-left: 100px;" on:click="{ignore}">
      はいはい（無視）
    </Button>
  </div>
</Modal>
{:else if $success}
<Modal>
  <div class="success">
    {$success}
  </div>
</Modal>
{/if}

<style>
  .error {
    color: red;
  }
  .success {
    color: green;
  }

  fieldset {
    margin-top: 20px;
    width: 400px;
    text-align: left;
  }
  .message {
    text-align: center;
  }

  .btns {
    margin-top: 30px;
  }
</style>
