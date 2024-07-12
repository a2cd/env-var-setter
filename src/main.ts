import {createApp} from "vue";
import "./styles.css";
import App from "./App.vue";
import components from './components'

// import router from './router'
// import store from './store'
// import i18n from './locale'

// import directive from './directive'


const app = createApp(App)
// app.use(router)
// app.use(store)
// app.use(i18n)
app.use(components)
// app.use(directive)

app.mount('#app')
