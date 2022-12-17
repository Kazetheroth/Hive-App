<script>
  import Stock from "./Stock.svelte";

</script>

<main id="base" class="container">
  <div class="row">
    <img class="logo" src="/hive.png" />
  </div>
  <div class="row">
    <button class="hive-btn">
      Gestion des Stocks
    </button>
    <button class="hive-btn">
      Comptabilité
    </button>
    <button class="hive-btn">
      Gestion des paies
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

  .hive-btn{
    margin: 10px 5px;
  }

</style>
