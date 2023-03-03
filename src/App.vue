<template>
  <div class="flex-col flex-fill" style="padding: 4px">
    <div style="height: 28px" @mousedown.self="onMove">
      <button class="btn-round" @click.stop="onPin" :style="pin ? {color:'#18a058'} : null">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 16 16">
          <g fill="none">
            <path
                d="M10.059 2.445a1.5 1.5 0 0 0-2.386.354l-2.02 3.79l-2.811.937a.5.5 0 0 0-.196.828L4.793 10.5l-2.647 2.647L2 14l.853-.146L5.5 11.207l2.146 2.147a.5.5 0 0 0 .828-.196l.937-2.81l3.779-2.024a1.5 1.5 0 0 0 .354-2.38L10.06 2.444z"
                fill="currentColor"></path>
          </g>
        </svg>
      </button>
    </div>
    <div class="flex-row card flex-fill">
      <textarea class="flex-fill" rows="1" v-model="src" placeholder="输入内容后按下Enter翻译"/>
    </div>
    <div class="dst" :class="trigger ? 'expanded' : 'flex-fill'" style="position: relative; overflow: hidden" @mouseover="trigger = true" @mouseleave="trigger = false">
      <div style="max-height: 100%; margin: 0 4px; position: absolute; overflow-y: scroll">
        {{ dst }}
      </div>
    </div>
  </div>
</template>

<script setup>
import {listen} from "@tauri-apps/api/event"
import {appWindow} from '@tauri-apps/api/window';
import {onMounted, onUnmounted, reactive, ref} from "vue";
import baidu from "./api/baidu.js";
import {invoke} from "@tauri-apps/api/tauri";

const events = reactive({})
const src = ref("")
const dst = ref("")
const pin = ref(false)
const loading = ref(false)
let moving = false;
const trigger = ref(true)

async function onMove() {
  moving = true;
  await invoke("start_move", {label: "main"});
}

onMounted(async () => {
  events.blur = await listen("tauri://blur", async (e) => {
    if (pin.value || moving) return;
    await appWindow.hide();
  })
  events.moven = await listen("tauri://move", (e) => {
    if (e.payload === "end") moving = false;
  })
  events.translate = await listen("main://translate", (e) => {
    src.value = e.payload
    if (src.value.trim() === "") return
    loading.value = true
    baidu(src.value.trim()).then(value => {
      dst.value = value.trans_result.map(d => d.dst).join("\n")
    }).catch(reason => dst.value = reason).finally(() => {
      loading.value = false
    })
  })
})

onUnmounted(async () => {
  events.blur()
  events.translate()
})

function onPin() {
  console.log("onPin")
  pin.value = !pin.value
}

</script>

<style scoped>
textarea {
  padding: 1px;
  resize: none;
  font-size: 14px;
  font-family: v-sans, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol";
  line-height: 1.4;
  border-width: 0;
  background-color: transparent;
  min-height: 16px;
}

textarea:focus {
  outline: none !important;
}

.card {
  border-radius: 4px;
  background-color: #0000000d;
  padding: 6px;
}

.dst{
  transition: all 0.4s ease-in-out;
}

.expanded {
  flex: 2;
}

.card.collapsed{
  margin-top: 4px;
  overflow-y: hidden;
  height: 16px;
}

</style>
