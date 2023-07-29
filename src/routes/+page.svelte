<script lang='ts'>
  import { invoke } from '@tauri-apps/api/tauri'

  async function setPower(status: Boolean) {
    return await invoke('set_power', { status })
  }

</script>

<button on:click={() => setPower(true)}>LIGHT ON</button>
<button on:click={() => setPower(false)}>LIGHT OFF</button>