<template>
  <div class="flex-row">
    <div class="menu">
      <button class="menu-item" :class="{'active':active===0}" v-click="()=>active=0">
        基础设置
      </button>
      <button class="menu-item" :class="{'active':active===1}" v-click="()=>active=1">
        翻译接口
      </button>
      <button class="menu-item" :class="{'active':active===2}" v-click="()=>active=2">
        OCR接口
      </button>
      <button class="menu-item" :class="{'active':active===3}" v-click="()=>active=3">
        软件信息
      </button>
    </div>
    <div class="content">
      <n-card v-if="active===0" title="基础设置">
        <n-form label-placement="left" label-width="auto" require-mark-placement="right-hanging">
          <n-form-item label="开机自启" path="switchValue">
            <n-switch v-model:value="autoStart"/>
          </n-form-item>
          <n-form-item label="窗口宽度" path="switchValue" >
            <n-slider v-model:value="windowWidth" :step="5" :min="280" :max="560"/>
          </n-form-item>
          <n-form-item label="窗口高度" path="switchValue">
            <n-slider v-model:value="windowHeight" :step="5" :min="168" :max="336"/>
          </n-form-item>

        </n-form>
      </n-card>
    </div>
  </div>
</template>

<script setup>
import {nextTick, onMounted, onUnmounted, reactive, ref} from "vue";
import {NCard,NForm, NFormItem, NSwitch,NSlider} from "naive-ui";

const active = ref(0)
const autoStart = ref(false)
const windowWidth = ref(280)
const windowHeight = ref(168)

function onMenuTap(val) {
  active.value = val
}

</script>

<style scoped>
.menu {
  width: 80px;
  padding: 4px;
  align-items: center;
  display: flex;
  flex-direction: column;
}

.menu-item {
  color: #333639;
  height: 40px;
  width: inherit;
  margin-top: 6px;
  padding: 6px;
  background: transparent;
  outline: 0;
  border-radius: 4px;
  border-width: 0;
  /*font-weight: bold;*/
  display: flex;
  justify-content: center;
  align-items: center;
}

.menu-item:hover {
  background: #0000000d;
}

.menu-item.active {
  background: #0000000d;
  color: var(--pColor);
}
.content{
  display: flex;
  flex-direction: column;
  flex: 1;
}
.n-card{
  margin: 8px;
  flex-shrink: 0
}
</style>