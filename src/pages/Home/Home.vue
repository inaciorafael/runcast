<script lang="ts">
import { defineComponent } from 'vue'

import HomeHeaderVue from '../../components/HomeHeader.vue'
import HomeGamesSelect from '../../components/HomeGamesSelect.vue'

export default defineComponent({
  name: 'Home',
  components: {
    HomeHeaderVue,
    HomeGamesSelect,
  },
  data() {
    return ({
      title: '',
    })
  },
})
</script>

<template>
  <HomeHeaderVue />
  <HomeGamesSelect />
</template>
