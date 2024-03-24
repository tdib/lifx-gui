<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"
    import ColourPicker, { type HsvaColor, type RgbaColor } from "svelte-awesome-color-picker"

    interface HSBK {
        hue: number,
        saturation: number,
        brightness: number,
        kelvin: number,
    }

    interface Colour {
        hsbk: HSBK,
        duration: number
    }

    export let macAddress: string
    export let lightName: string

    let hsv: HsvaColor
    let kelvin = 0
    let duration = 0
    let colour: Colour

    $: if (hsv) {
        colour = {
            hsbk: {
                hue: Math.round(hsv.h * (65535 / 360)),
                saturation: Math.round(hsv.s * (65535 / 100)),
                brightness: Math.round(hsv.v * (65535 / 100)),
                kelvin: Math.round(kelvin)
            },
            duration: Math.round(duration)
        }
    }

    $: macAddress = macAddress.replaceAll(":", "")

    function debounce<T extends (...args: any[]) => any>(func: T, wait: number): (...args: Parameters<T>) => void {
        let timeout: any

        return function(...args: Parameters<T>) {
            if (timeout !== null) {
                clearTimeout(timeout)
            }
            timeout = setTimeout(() => {
                func(...args)
            }, wait)
        }
    }
    const debouncedSetColour = debounce(setColour, 20)

    async function setPower(status: Boolean) {
        return await invoke("set_power", { mac: macAddress, status })
    }

    async function setColour(colour: Colour) {
        return await invoke("set_colour", {
            mac: macAddress,
            hue: colour.hsbk.hue,
            saturation: colour.hsbk.saturation,
            brightness: colour.hsbk.brightness,
            kelvin: colour.hsbk.kelvin,
            duration: colour.duration
        })
    }
</script>

<div class="light-item-container">
    <button on:click={() => setPower(true)}>{lightName} On</button>
    <button on:click={() => setPower(false)}>{lightName} Off</button>
    <ColourPicker bind:hsv on:input={() => {
        debouncedSetColour(colour)
    }}></ColourPicker>
</div>

<style lang="scss">
    .light-item-container {
        padding-block: 1em;
    }
</style>
