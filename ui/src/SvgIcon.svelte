<script lang="ts">
    import { svgIcons } from "./svgIcons"

    export let icon: string
    export let style: string = ""
    export let color: string ="#000"
    export let size: string = "20px"

    $:iconStr = () => {
        let i = svgIcons[icon]
        if (!i) console.log("missing icon for ", icon)
        if (color) {
            i = i.replace("<svg",`<svg fill="${color}"`)
        }
        return `data:image/svg+xml;charset=utf-8,${i.replace("#","%23")}`
    }
    $:styleStr = `${size ? "width:"+size+";height:"+size+";":""};`
</script>

<div class="icon-outer" style={style ? style : styleStr}><img width={size} src={iconStr()}/></div>

<style>
    .icon-outer {
        display: inline-block;
        height: auto;
    }
    .icon img {
        vertical-align: middle;
    }
</style>