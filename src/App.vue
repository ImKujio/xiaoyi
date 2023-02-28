<template>
  <div class="flex-col flex-fill" style="padding: 4px">
    <div data-tauri-drag-region style="height: 28px">
      <button class="btn-round" @click="onPin" :style="pin ? {color:'#18a058'} : null">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 16 16"><g fill="none"><path d="M10.059 2.445a1.5 1.5 0 0 0-2.386.354l-2.02 3.79l-2.811.937a.5.5 0 0 0-.196.828L4.793 10.5l-2.647 2.647L2 14l.853-.146L5.5 11.207l2.146 2.147a.5.5 0 0 0 .828-.196l.937-2.81l3.779-2.024a1.5 1.5 0 0 0 .354-2.38L10.06 2.444z" fill="currentColor"></path></g></svg>
      </button>
      <button class="btn-round" @click="open">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 16 16"><g fill="none"><path d="M8.5 2.75a.75.75 0 0 0-1.5 0V7H2.75a.75.75 0 0 0 0 1.5H7v4.25a.75.75 0 0 0 1.5 0V8.5h4.25a.75.75 0 0 0 0-1.5H8.5V2.75z" fill="currentColor"></path></g></svg>
      </button>
      <button class="btn-round" @click="close">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 16 16"><g fill="none"><rect x="3" y="7.25" width="10" height="1.5" rx=".75" fill="currentColor"></rect></g></svg>
      </button>
    </div>
    <div class="flex-row card" style="margin-top: 4px">
      <textarea class="flex-fill" rows="4" v-model="src" style="height: 80px" placeholder="输入内容后按下Enter翻译"/>
    </div>
    <div class="flex-row card" style="margin-top: 4px;padding: 0">
      <n-button quaternary @click="onUpper" size="small">
        英→中
      </n-button>
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
import {Pin24Regular, Pin24Filled, Add24Regular, Subtract24Regular} from "@vicons/fluent";
import {NButton, NIcon} from 'naive-ui'
import baidu from "./api/baidu.js";
import LoadingView from "./components/LoadingView.vue";

const events = reactive({})
const src = ref("")
const dst = ref("")
const pin = ref(false)
const drag = ref(false)
const size = reactive({width: 0, height: 0})
const loading = ref(false)
let interval = false
let int = null;

function onLower() {
  int = setInterval(lower, 10)
}

function lower() {
  if (size.height <= 280) {
    clearInterval(int)
    int = null
    return
  }
  size.height -= 20
}

function onUpper() {
  if (size.height >= 600) return
  int = setInterval(upper, 1)
}

function upper() {
  if (size.height > 600) {
    clearInterval(int)
    int = null
    return
  }
  size.height += 20
}

function close(){
  if (interval) return
  interval = true
  const int = setInterval(()=>{
    if (size.height <= 168) {
      interval = false
      clearInterval(int)
      return
    }
    size.height -= 10
  }, 1)
}

function open(){
  console.log('interval',interval)
  if (interval) return
  interval = true
  appWindow.outerSize().then(value => {
    size.height = value.height;
    size.width = value.width;
    const int = setInterval(()=>{
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

function onDragEnd() {
  console.log("drag end")
  drag.value = false
}


async function onDrag() {
  drag.value = true
  await appWindow.startDragging();
}

onMounted(async () => {
  appWindow.outerSize().then(value => {
    size.height = value.height;
    size.width = value.width;
  })
  events.blur = await listen("tauri://blur", async (e) => {
    console.log("tauri://blur")
    if (pin.value || drag.value) return;
    await appWindow.hide();
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
  events.focus()
  events.blur()
  events.translate()
})

function onPin() {
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
