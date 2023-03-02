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
    <div class="flex-row card" style="margin-top: 4px">
      <textarea class="flex-fill" rows="4" v-model="src" style="height: 80px" placeholder="输入内容后按下Enter翻译"/>
    </div>
    <div class="flex-row card" style="margin-top: 4px;padding: 0">
      <button class="btn-rect">英→中</button>
      <loading-view v-if="loading"/>
    </div>
    <div class="flex-fill" style="position: relative; overflow: hidden">
      <div style="max-height: 100%; margin: 0 4px; position: absolute; overflow-y: scroll">
        {{ dst }}
      </div>
    </div>
  </div>
</template>

<script setup>
import {listen} from "@tauri-apps/api/event"
import {appWindow, PhysicalSize} from '@tauri-apps/api/window';
import {onMounted, onUnmounted, reactive, ref, watch} from "vue";
import baidu from "./api/baidu.js";
import LoadingView from "./components/LoadingView.vue";
import {invoke} from "@tauri-apps/api/tauri";

const events = reactive({})
const src = ref("")
const dst = ref("")
const pin = ref(false)
const size = reactive({width: 0, height: 0})
const loading = ref(false)
let moving = false;
let interval = false

async function onMove() {
  moving = true;
  await invoke("start_move",{label:"main"});
}

function test() {
  appWindow.innerSize().then(value => {
    console.log(value)
  })
}

function open() {
  if (interval) return
  interval = true
  appWindow.innerSize().then(value => {
    size.height = value.height;
    size.width = value.width;
    console.log(value)
    const int = setInterval(() => {
      if (size.height >= 288) {
        interval = false
        clearInterval(int)
        return
      }
      size.height += 10
    }, 1)
  })
}

watch(size, async (n) => {
  appWindow.setSize(new PhysicalSize(n.width, n.height)).then()
})

onMounted(async () => {
  appWindow.innerSize().then(value => {
    size.height = value.height;
    size.width = value.width;
    console.log(value)
  })
  events.blur = await listen("tauri://blur", async (e) => {
    if (pin.value || moving) return;
    await appWindow.hide();
  })
  events.moven = await listen("tauri://move",(e) =>{
    if (e.payload === "end") moving = false;
  })
  events.translate = await listen("translate", (e) => {
    src.value = e.payload
    if (src.value.trim() === "") return
    loading.value = true
    baidu(src.value.trim()).then(value => {
      dst.value = value.trans_result.map(d => d.dst).join("\n")
    }).catch(reason => dst.value = reason).finally(() => {
      loading.value = false
      open()
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
  resize: none;
  font-size: 14px;
  font-family: v-sans, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol";
  line-height: 1.4;
  border-width: 0;
  background-color: transparent;
}

textarea:focus {
  outline: none !important;
}

.card {
  border-radius: 4px;
  background-color: #0000000d;
  padding: 6px;
}
</style>
