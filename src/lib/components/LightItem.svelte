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

    let hue = 0
    let saturation = 0
    let brightness = 0
    let kelvin = 0
    let duration = 0
    let colour: Colour
    let log: string

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
    // $: console.log("hsv is ", hsv, hue, saturation, brightness)

    $: macAddress = macAddress.replaceAll(":", "")

    function debounce<T extends (...args: any[]) => any>(func: T, wait: number): (...args: Parameters<T>) => void {
        let timeout: any

        return function(...args: Parameters<T>) {
            if (timeout !== null) {
                clearTimeout(timeout)
            }
            timeout = setTimeout(() => {
                func(...args);
            }, wait)
        }
    }
    const debouncedSetColour = debounce(setColour, 200)
    

    async function setPower(status: Boolean) {
        // console.log("what");
        // log += "setting power\n"
        return await invoke("set_power", { mac: macAddress, status })
    }

    async function setColour(colour: Colour) {
        // console.log("Here?", colour);
        // log += "setting COLOUR " + JSON.stringify(colour) + " " + macAddress
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
        // console.log("yo");
        debouncedSetColour(colour)
    }}></ColourPicker>
    <button on:click={() => {
        setColour(colour)
    }}>Set colour!</button>

    <p>{log}</p>
</div>

<style lang="scss">
    .light-item-container {
        padding-block: 1em;
    }

</style>
