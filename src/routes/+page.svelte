<script lang="ts">
    import devicesJson from "../lib/data/devices.json"
    import LightItem from "$lib/components/LightItem.svelte"
    
    interface Device {
        name: string,
        mac: string,
        ip: string
    }
    
    let devices: Device[] = devicesJson
    
</script>

<main style="padding: 1.5em;">
    <h1>Light Controller</h1>

    {#each devices as device}
        <LightItem macAddress={device.mac} lightName={device.name} />
    {/each}
</main>