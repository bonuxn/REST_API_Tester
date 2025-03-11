<script lang="ts">
import { defineComponent, ref, onMounted, nextTick } from 'vue';

export default defineComponent({
  setup() {
    const logs = ref<string[]>([]);
    const logContainer = ref<HTMLElement | null>(null);

    const scrollToBottom = () => {
      nextTick(() => {
        if (logContainer.value) {
          logContainer.value.scrollTop = logContainer.value.scrollHeight;
        }
      });
    };
    const addTestLog = () => {
      const timestamp = new Date().toLocaleTimeString();
      logs.value.push(`Test log: ${timestamp}`);
      scrollToBottom();
    };
    onMounted(() => {
      // 例: WebSocket からログを受信
      const socket = new WebSocket('ws://localhost:8080');
      socket.onmessage = (event) => {
        logs.value.push(event.data);
        scrollToBottom();
      };
    });

    return {
      logs,
      logContainer,
      scrollToBottom,
      addTestLog
    };
  },
});
</script>

<template>
  <div class="communication-log">
    <div ref="logContainer" class="log-container">
      <div v-for="(log, index) in logs" :key="index" class="log-item">
        {{ log }}
      </div>
    </div>
    <button @click="addTestLog">Add Test Log</button>
  </div>
</template>

<style scoped>
.communication-log {
  width: 800px;
  height: 400px;
  border: 1px solid #ccc;
  overflow: visible;
  position: relative;
}

.log-container {
  height: 100%;
  overflow-y: auto;
  padding: 10px;
}

.log-item {
  padding: 5px;
  border-bottom: 1px solid #eee;
}

.add-button {
  position: absolute;
  bottom: 10px;
  left: 10px;
  z-index: 10;
}
</style>