import {createApp} from "vue";
import "./style.css";
import NProgress from "nprogress";
import "nprogress/nprogress.css";
import Main from "./main.vue";

NProgress.configure({showSpinner: false,})

// 阻止 Ctrl + F/G/P
window.addEventListener('keydown', function(event) {
    if (event.ctrlKey && "fgp".includes(event.key)) {
        event.preventDefault();
    }
});

const app = createApp(Main)
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