<template>
  <v-app>
    <v-main>
      <SpeedDashboard />
    </v-main>
  </v-app>
</template>

<script>
import SpeedDashboard from "./components/SpeedDashboard.vue";

export default {
  name: "App",
  components: {
    SpeedDashboard,
  },
};
</script>
