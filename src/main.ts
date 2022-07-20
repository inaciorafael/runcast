import { createApp } from 'vue'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

import App from './App.vue'

createApp(App)
  .component('v-icon', FontAwesomeIcon)
  .mount('#app')
