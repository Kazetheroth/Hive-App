<script>
  import Stock from "./Stock.svelte";

</script>

<main id="base" class="container">
  <div class="row">
    <img class="logo" src="/hive.png" />
  </div>
  <div class="row">
    <button>
      Stock
    </button>
  </div>
  <Stock />
</main>


<style>
  .logo{
    margin: auto;
    height: 150px!important;
    width: 150px!important;
  }
  .logo:hover {

    filter: drop-shadow(0 0 2em #747bff);
  }

</style>
