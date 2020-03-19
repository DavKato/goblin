<script>
  import ItemList from './ExplorerMode/ItemList.svelte'
  import Explorer from './ExplorerMode/Explorer.svelte'
</script>

<main>
  <Explorer></Explorer>
  <ItemList></ItemList>
</main>

<style>
  main {
    display: flex;
    width: 100vw;
    height: 100vh;
  }
</style>
