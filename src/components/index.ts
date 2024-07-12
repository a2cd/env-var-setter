import {Icon} from '@arco-design/web-vue';
import {App} from "vue";

import Greet from "@/components/greet/index.vue";

// 获得iconfont.js文件的网络位置
const iconfontUrl = new URL('@/assets/iconfont.js', import.meta.url).href
const IconFont = Icon.addFromIconFontCn({src: iconfontUrl});



export default {
    install(Vue: App) {
        Vue.component('IconFont', IconFont)
        Vue.component('Greet', Greet)
    },
}
