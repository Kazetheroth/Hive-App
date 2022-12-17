<script>
    import {Entity, EntityType} from "./js/Entity.js";
    import {invoke} from "@tauri-apps/api/tauri";
    const entityType = new EntityType();
    const entity = new Entity();
    entityType.name = "Inconnu";

    function handleSelect(event) {
        console.log('selected item', event.target.selectedOptions[0].value);
        entityType.name = event.target.selectedOptions[0].value
    }

    async function create_stock_entity(){
        entity.entity_type = entityType;
        const stockString = JSON.stringify(entity);
        console.log(stockString);
        const msg = await invoke("create_stock_entity", { stockString });
        console.log(msg);
    }

    async function handle_stock_load(){
        console.log("loading...")
        const msg = await invoke("handle_stock_load");
        console.log(msg);
    }
</script>

<div class="container" on:loadstart={handle_stock_load}>
    <h2>Créer un objet</h2>
    <div class="row">
        <input id="entity-name" placeholder="Nommez votre objet" bind:value={entity.name} class="entity-ipt">
        <div>
            <h3 class="entity-ipt">Type d'objet : </h3>
            <select id="entity-type-select" on:change={handleSelect}  class="entity-ipt">
                <option value="Inconnu" selected>Inconnu</option>
                <option value="Vêtement">Vêtement</option>
            </select>
        </div>

    </div>
    <div class="row">
        <button on:click={create_stock_entity}  class="entity-ipt">
            Envoyer
        </button>
    </div>
</div>

<style>
    .entity-ipt{
        margin: 0.5rem;
    }
</style>
