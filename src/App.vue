<template>
  <div class="flex-col flex-fill">
    <div data-tauri-drag-region style="margin: 4px">
      <n-button quaternary circle @click="onPin" size="small" :type="pin ? 'success' : 'default'">
        <template #icon>
          <n-icon>
            <Pin24Regular v-if="!pin"/>
            <Pin24Filled v-if="pin"/>
          </n-icon>
        </template>
      </n-button>
      <n-button quaternary circle @click="onUpper" size="small">
        <template #icon>
          <n-icon>
            <Add24Regular/>
          </n-icon>
        </template>
      </n-button>
      <n-button quaternary circle @click="onLower" size="small">
        <template #icon>
          <n-icon>
            <Subtract24Regular/>
          </n-icon>
        </template>
      </n-button>
    </div>
    <div class="flex-row card" style="margin-left: 4px;margin-right: 4px">
      <textarea class="flex-fill" rows="4" v-model="src" placeholder="输入内容后按下Enter翻译"/>
    </div>
    <div class="flex-fill card" style="margin: 4px;padding: 4px;position: relative;overflow: hidden">
      <div style="width: 100%;height: 100%;position: absolute;overflow: scroll">
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

const events = reactive({})
const src = ref("")
const dst = ref("")
const pin = ref(false)
const drag = ref(false)
const size = reactive({width: 0, height: 0})

let int = null;

function onLower() {
  int = setInterval(lower, 10)
}

function lower(){
  if (size.height < 280) {
    clearInterval(int)
    int = null
    return
  }
  size.height -= 20
}

function onUpper() {
  if (size.height > 600) return
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
    dst.value = "翻译中..."
    baidu(src.value.trim()).then(value => {
      dst.value = value.trans_result.map(d => d.dst).join("\n")
    }).catch(reason => dst.value = reason)
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
  padding: 6px;
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
  border-width: 0;
  border-radius: 6px;
  background-color: #ffffff;
}
</style>
