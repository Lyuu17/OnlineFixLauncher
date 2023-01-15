<script setup lang="ts">
import { ServerDataList } from '../API';

defineProps<{
  data: ServerDataList
}>();

const emits = defineEmits(["game_launch"]);
</script>

<template>
  <div>
    <table class="m-auto"
      style="border-spacing: 0;">
      <thead>
        <tr>
          <th class="center">#</th>
          <th class="center">Name</th>
          <th class="center">Capacity</th>
          <th class="center">Game</th>
          <th class="center"></th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="e, i in data" :key="i">
          <td>{{ i }}</td>
          <td>{{ e.ServerName }}</td>
          <td>{{ e.ServerCapacity }}/{{ e.MaxCapacity }}</td>
          <td>{{ e.GameName }}</td>
          <td>
            <button @click="emits('game_launch', e.GameName, e.ServerId)">
              Join
            </button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
td {
  border: 1px solid whitesmoke;
  border-left: 0;
  border-right: 0;
  border-bottom: 0;
  padding: 10px;
}
</style>