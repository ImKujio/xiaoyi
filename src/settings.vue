<template>
  <div class="flex-col">
    <div style="height: 32px;display: flex;flex-direction: row;align-items: center">
      <svg-icon style="width: 16px;height: 16px;margin:0 8px" name="logo"/>
      <span>小译设置</span>
    </div>
    <div data-tauri-drag-region style="height: 32px; width: 100%;position: absolute;top: 0"/>
    <button  class="btn-rect inactive" style="position:absolute;top:4px;right:4px;padding: 6px 12px" v-click="async () => {await appWindow.close()}">
      <svg-icon style="width: 16px;height: 16px" name="close"/>
    </button>
    <div style="position: absolute;top:0;left: 50%;transform:translateX(-50%);display: flex;flex-direction: row;align-items: center">
      <button class="btn-rect left trans-tab" :class="{active:active===0,inactive:active!==0}" v-click="() => active=0">基础设置</button>
      <button class="btn-rect left trans-tab" :class="{active:active===1,inactive:active!==1}" v-click="() => active=1">翻译接口</button>
      <button class="btn-rect left trans-tab" :class="{active:active===2,inactive:active!==2}" v-click="() => active=2">OCR接口</button>
      <button class="btn-rect left trans-tab" :class="{active:active===3,inactive:active!==3}" v-click="() => active=3">关于</button>
    </div>
    <div class="flex-col">
      <n-scrollbar style="flex: 1">
        <div v-if="active === 0" class="flex-col">
          <n-form label-width="auto" label-placement="left" style="margin: 16px">
            <n-form-item label="开机自启">
              <n-switch :value="autoStart" @update:value="onAutoStart"/>
            </n-form-item>
          </n-form>
        </div>
      </n-scrollbar>
    </div>
  </div>
</template>

<script setup>
import {ref} from "vue";
import {appWindow} from "@tauri-apps/api/window";
import { enable, isEnabled, disable } from "tauri-plugin-autostart-api";
import {NForm,NFormItem,NSwitch,NScrollbar} from "naive-ui";

const active = ref(0)
const autoStart = ref(false)

isEnabled().then(value => autoStart.value = value)

async function onAutoStart(val) {
  if (val) await enable()
  else await disable()
  autoStart.value = await isEnabled()
}


</script>

<style scoped>
button.active {
  font-weight: bold;
  color: var(--pColor);
}

button.inactive {
  color: #727272;
}

.trans-tab{
  transition: color,font-size 0.2s linear;
}

</style>