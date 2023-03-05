<template>
  <div class="content">
    <div id="title">
      <button class="btn-round left" v-click="onPin" :style="pin ? {color:'#18a058'} : null">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 16 16">
          <g fill="none">
            <path
                d="M10.059 2.445a1.5 1.5 0 0 0-2.386.354l-2.02 3.79l-2.811.937a.5.5 0 0 0-.196.828L4.793 10.5l-2.647 2.647L2 14l.853-.146L5.5 11.207l2.146 2.147a.5.5 0 0 0 .828-.196l.937-2.81l3.779-2.024a1.5 1.5 0 0 0 .354-2.38L10.06 2.444z"
                fill="currentColor"></path>
          </g>
        </svg>
      </button>
      <button class="btn-round left" v-click="onTraget">
        {{ query.target[target].name }}
      </button>
      <div style="flex: 1" @mousedown.self="onMove"/>
      <button :key="key" class="btn-round right" :class="{'hide':origin,'trans-btn':trans}" v-click="onInsert">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 20 20" style="rotate: 90deg;">
          <g fill="none">
            <path
                d="M4 4a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4zm0 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-2a2 2 0 0 0-2-2H6a2 2 0 0 0-2 2v2zM2.5 9.5a.5.5 0 0 0 0 1h15a.5.5 0 0 0 0-1h-15z"
                fill="currentColor"></path>
          </g>
        </svg>
      </button>
      <button :key="key" class="btn-round right trans-btn" :class="{'hide':origin,'trans-btn':trans}" :style="copied ? {color:'#18a058'} : null"
              v-click="onCopy">
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 16 16">
          <g fill="none">
            <path
                d="M4 4.085V10.5a2.5 2.5 0 0 0 2.336 2.495L6.5 13h4.414A1.5 1.5 0 0 1 9.5 14H6a3 3 0 0 1-3-3V5.5a1.5 1.5 0 0 1 1-1.415zM11.5 2A1.5 1.5 0 0 1 13 3.5v7a1.5 1.5 0 0 1-1.5 1.5h-5A1.5 1.5 0 0 1 5 10.5v-7A1.5 1.5 0 0 1 6.5 2h5z"
                fill="currentColor"></path>
          </g>
        </svg>
      </button>
      <button class="btn-round right" :disabled="origin && !dst" v-click="onTrans">
        {{ origin ? "原" : "译" }}
      </button>
    </div>
    <flex-wrapper :key="key" :class="{'trans-page':trans,'fade':!origin,'fill':origin}" padding="4px">
      <textarea v-focus="origin" ref="inputRef" rows="1" v-model="src" placeholder="输入内容后按下Enter翻译" @keydown.enter.prevent="onTranslate"/>
    </flex-wrapper>
    <flex-wrapper :key="key+1000" :class="{'trans-page':trans,'fade':origin,'fill':!origin}" padding="4px">
      <div style="box-sizing: border-box;padding: 4px;width: 100%;height: 100%;overflow-y: scroll;white-space: pre-wrap;">
        {{ dst }}
      </div>
    </flex-wrapper>
  </div>
</template>

<script setup>
import {listen} from "@tauri-apps/api/event"
import {invoke} from "@tauri-apps/api/tauri";
import {writeText} from '@tauri-apps/api/clipboard';
import {nextTick, onMounted, onUnmounted, reactive, ref} from "vue";
import dict from "./api/dict.js";
import FlexWrapper from "./components/FlexWrapper.vue";
import NProgress from "nprogress"
import query from "./api/query.js";

const events = reactive({})
const src = ref("")
const dst = ref("")
const pin = ref(false)
const origin = ref(true)
const trans = ref(true)
const key = ref(0)
const target = ref(0)
const copied = ref(false)


async function onTranslate() {
  if (src.value.trim() === "") return
  copied.value = false
  NProgress.start()
  let rst = await dict.query(src.value.trim())
  if (rst == null) {
    try {
      rst = await query.query(src.value.trim(), query.target[target.value].value);
    } catch (e) {
      rst = [e.message]
    }
  }
  dst.value = rst.join("\n");
  NProgress.done()
  origin.value = false
}

async function onMove() {
  await invoke("start_move", {label: "main"});
}

onMounted(async () => {
  events.translate = await listen("main://translate", async (e) => {
    src.value = e.payload.trim()
    reset()
    if (src.value === "") inputRef.value.focus()
    await onTranslate(true)
  })
})

onUnmounted(async () => {
  events.translate()
})

async function onPin() {
  pin.value = !pin.value
  await invoke("state_set", {key: "main://pin", val: pin.value ? "1" : "0"})
}

function onTrans() {
  origin.value = !origin.value
}

function onTraget() {
  target.value = (target.value + 1) % query.target.length
  if (!origin.value) onTranslate()
}

async function onCopy() {
  await writeText(dst.value);
  copied.value = true
}

async function onInsert() {
  await invoke("insert", {label: "main", text: dst.value})
}

function reset() {
  trans.value = false;
  key.value++
  nextTick(() => {
    origin.value = true;
    trans.value = true;
  })
}
</script>

<style scoped>
textarea {
  height: 100%;
  width: 100%;
  padding: 4px;
  resize: none;
  font-size: 14px;
  font-family: v-sans, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol";
  line-height: 1.4;
  border-width: 0;
  color: inherit;
  background-color: transparent;
  outline: none !important;
  box-sizing: border-box;
}

#title {
  display: flex;
  flex-direction: row;
}

#title .left {
  margin-top: 4px;
  margin-left: 4px;
}

#title .right {
  margin-top: 4px;
  margin-right: 4px;
}

.trans-page {
  transition: flex 300ms cubic-bezier(0, .6, .5, 1),
  color 150ms linear;
}

.trans-btn {
  transition: visibility 150ms linear, opacity 150ms linear;
}

.fade {
  color: transparent !important;
}

.hide {
  visibility: hidden;
  opacity: 0;
}

.fill {
  flex: 1;
}

</style>
