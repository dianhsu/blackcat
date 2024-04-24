import { createApp } from "vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import "./styles.css";
import App from "./App.vue";
import HomeIndex from './components/home/HomeIndex.vue'
import AboutIndex from './components/about/AboutIndex.vue'
import ProcessIndex from './components/process/ProcessIndex.vue'
import FileIndex from './components/file/FileIndex.vue'
import ScriptIndex from './components/script/ScriptIndex.vue'
import TerminalIndex from './components/terminal/TerminalIndex.vue'
import UtilityIndex from './components/utility/UtilityIndex.vue'
import { createRouter, createWebHashHistory } from 'vue-router'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'
const routes = [
  {
    path: '/',
    name: 'Home',
    component: HomeIndex
  },
  {
    path: '/about',
    name: 'About',
    component: AboutIndex
  },
  {
    path: '/process',
    name: 'Process',
    component: ProcessIndex
  },
  {
    path: '/file',
    name: 'File',
    component: FileIndex
  },
  {
    path: '/script',
    name: 'Script',
    component: ScriptIndex
  },
  {
    path: '/terminal',
    name: 'Terminal',
    component: TerminalIndex
  },
  {
    path: '/utility',
    name: 'Utility',
    component: UtilityIndex
  }
]
const router = createRouter({
  history: createWebHashHistory(),
  routes
})

const app = createApp(App);
app.use(router);
app.use(ElementPlus);
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}
app.mount("#app");
