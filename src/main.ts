import { createApp } from 'vue'

import App from './App.vue'

import Space from './components/Space.vue'

createApp(App)
  .component('Space', Space)
  .mount('#app')
