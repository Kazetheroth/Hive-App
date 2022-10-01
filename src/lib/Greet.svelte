<script>
  import { invoke } from "@tauri-apps/api/tauri"

  let name = "";
  let greetMsg = ""

  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name })
    await test_medhi();
  }

  async function test_medhi() {
    const testMsg = await invoke("test_medhi", { name });
    console.log(await testMsg);
  }
</script>

<div>
  <div class="row">
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button on:click={test_medhi}>
      Greet
    </button>
  </div>
  <p>{greetMsg}</p>
</div>

