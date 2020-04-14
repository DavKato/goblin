<script>
  export let css = '';
  export let color = 'native';
  export let type = '';
  let className = '';
  export { className as class };

  const colors = {
    native: 'var(--cl-frame-bg)',
    red: '#710917',
    blue: '#20206e'
  };

  $: style = `--bg:${colors[color]}; ${css}`;
</script>

<button class="{className} shadow" {style} {type} on:click>
  <slot></slot>
</button>

<style>
  button {
    border-radius: 5px;
    padding: 4px 10px;
    color: var(--cl-stdout-txt);
    background-color: var(--bg);
  }
  button:hover {
    background: linear-gradient(rgba(0, 0, 0, 0.25), rgba(0, 0, 0, 0.25))
      var(--bg);
  }
</style>
