<script setup lang="ts">
import { fetch } from '@tauri-apps/api/http';
import { invoke } from '@tauri-apps/api'
import { onMounted, ref } from 'vue';
import { API_URL } from './API';

import ServerList from './components/ServerList.vue'

const curServerList = ref([]);

const refreshList = async () => {
  const response = await fetch(API_URL, {
    method: "GET",
    timeout: 3
  })
  .then((response) => response.data)
  .then((data: any) => {
    curServerList.value = data;
  })
  .catch((e) => {
    console.log(new Error(e));
  });
}

onMounted(async () => {
  await refreshList();
});

const game_launch = (game: string, server: string) => {
  invoke('game_launch', { game: game, server: server })
    .catch((error) => console.log(new Error(error)))
}

</script>

<template>
  <div>
    <h1 id="title">
      OnlineFix
    </h1>

    <div>
      <ServerList :data="curServerList" @game_launch="game_launch"/>
    </div>
  </div>
</template>

<style scoped>
#title {
  text-align: center;
}
</style>
