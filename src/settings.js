import {createApp} from "vue";
import "./style.css";
import Settings from "./settings.vue";
import SvgIcon from "./svg-icon/index.vue"

// 阻止 Ctrl + F/G/P
window.addEventListener('keydown', function (event) {
    if (event.ctrlKey && "fgp".includes(event.key)) {
        event.preventDefault();
    }
});

const app = createApp(Settings)

app.component("svg-icon", SvgIcon)

app.directive('click', {
    mounted(el, binding) {
        el.addEventListener('click', () => {
            if (el.__stop_click__) return;
            el.__stop_click__ = true;
            if (typeof binding.value === 'function') {
                binding.value()
            }else {
                eval(binding.value)
            }
            setTimeout(() => {
                el.__stop_click__ = false;
            }, binding.arg || 400)
        })
    }
})
app.mount("#app");