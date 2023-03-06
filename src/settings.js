import {createApp} from "vue";
import "./style.css";
import NProgress from "nprogress";
import "nprogress/nprogress.css";
import Settings from "./Settings.vue";
import "./init.js";

NProgress.configure({showSpinner: false,})

const app = createApp(Settings)
app.directive('click', {
    mounted(el, binding) {
        el.addEventListener('click', () => {
            if (el.__stop_click__) return;
            el.__stop_click__ = true;
            binding.value()
            setTimeout(() => {
                el.__stop_click__ = false;
            }, binding.arg || 500)
        })
    }
})

app.directive('focus', (el, binding) => {
    binding.value ? el.focus() : el.blur();
})
app.mount("#app");

