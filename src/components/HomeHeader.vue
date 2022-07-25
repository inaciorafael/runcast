<script lang="ts">
import { defineComponent } from 'vue'

import Clock from '../components/Clock.vue'
import Battery from '../components/Battery.vue'
import Network from '../components/Network.vue'
import Option from '../components/Option.vue'
import GameController from '../assets/icons/game-controller.png'
import Download from '../assets/icons/dowload.png'
import Settings from '../assets/icons/settings.png'
import PowerButton from '../assets/icons/power-button.png'
import Home from '../assets/icons/home.png'

export default defineComponent({
  name: 'HomeHeader',
  components: {
    Clock,
    Battery,
    Network,
    Option,
  },
  data() {
    return {
      GameController: GameController,
      Settings: Settings,
      PowerButton: PowerButton,
      Home: Home,
      Download: Download,
    }
  }
})
</script>

<template>
  <div class="wrapper">
    <div class="items-container">
      <Option>
        <img width="40" :src="Home" />
      </Option>
      <space horizontal="10" />
      <Option>
        <img width="40" :src="GameController" />
      </Option>
      <space horizontal="10" />
      <Option>
        <img width="40" :src="Settings" />
      </Option>
      <space horizontal="10" />
      <Option>
        <img width="40" :src="PowerButton" />
      </Option>
      <space horizontal="10" />
    </div>
    <div class="items-container">
      <clock />
      <space horizontal="20" />
      <network />
      <space horizontal="20" />
      <v-icon name="eye"></v-icon>
      <battery />
    </div>
  </div>
</template>

<style scoped>
.wrapper {
  grid-area: header;

  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  padding: 0 30px;
}

.items-container {
  display: flex;
  flex-direction: row;
  align-items: center;
}
</style>
