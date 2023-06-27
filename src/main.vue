<template>
  <div class="flex-col">
    <div id="title">
      <button class="btn-round left" v-click="onPin" :style="pin ? {color:'#18a058'} : null">
        <svg-icon class="icon" name="pin"/>
      </button>
      <button class="btn-round left" v-click="onTraget">
        {{ query.target[target].name }}
      </button>
      <div style="flex: 1" @mousedown.self="onMove"/>
      <button :key="key" class="btn-round right" :class="{'hide':origin,'trans-btn':trans}" v-click="onInsert">
        <svg-icon class="icon" style="rotate: 90deg" name="insert"/>
      </button>
      <button :key="key" class="btn-round right trans-btn" :class="{'hide':origin,'trans-btn':trans}" :style="copied ? {color:'#18a058'} : null"
              v-click="onCopy">
        <svg-icon class="icon" name="copy"/>
      </button>
      <button class="btn-round right" :disabled="origin && !dst" v-click="onTrans">
        {{ origin ? "原" : "译" }}
      </button>
    </div>
    <div :key="key" class="flex-fill-wrapper" :class="{'trans-page':trans,'fade':!origin,'fill':origin}">
      <div class="flex-fill-container">
        <textarea id="trans-input" v-focus="origin" ref="inputRef" rows="1" v-model="src" placeholder="输入内容后按下Enter翻译"
                  @keydown.enter.prevent="onEnter"/>
      </div>
    </div>
    <div :key="key" class="flex-fill-wrapper" :class="{'trans-page':trans,'fade':origin,'fill':!origin}">
      <div class="flex-fill-container">
        <div id="translation">
          {{ dst }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import {listen} from "@tauri-apps/api/event"
import {invoke} from "@tauri-apps/api/tauri";
import {writeText} from '@tauri-apps/api/clipboard';
import {nextTick, reactive, ref} from "vue";
import dict from "./dict.js";
import NProgress from "nprogress"
import query from "./query.js";
import SvgIcon from "./svg-icon/index.vue"

const events = reactive({})
const src = ref("")
const dst = ref("")
const pin = ref(false)
const origin = ref(true)
const trans = ref(true)
const key = ref(0)
const target = ref(0)
const copied = ref(false)

listen("main://translate", async (e) => {
  src.value = e.payload.trim()
  reset()
  if (src.value === "") return
  let auto = query.auto(src.value);
  if (auto !== -1) target.value = auto
  await translate(true)
}).then(value => events.translate = value)

async function translate() {
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
  origin.value = false
  NProgress.done()
}

async function onMove() {
  await invoke("start_move", {label: "main"});
}

async function onEnter() {
  if (src.value !== "") {
    let auto = query.auto(src.value);
    if (auto !== -1) target.value = auto
    await translate(true)
  }
}

async function onPin() {
  pin.value = !pin.value
  await invoke("state_post", {key: "main-pin", val: pin.value})
}

function onTrans() {
  origin.value = !origin.value
}

function onTraget() {
  target.value = (target.value + 1) % query.target.length
  if (!origin.value) translate()
}

async function onCopy() {
  await writeText(dst.value);
  copied.value = true
}

async function onInsert() {
  await invoke("insert", {text: dst.value})
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

#title .icon{
  width: 16px;
  height: 16px;
}

#trans-input {
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

#translation {
  box-sizing: border-box;
  padding: 4px;
  width: 100%;
  height: 100%;
  overflow-y: scroll;
  white-space: pre-wrap;
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


.trans-text {
  transition: all 0.3s;
}
</style>
