<template>
  <div>
    <h3>テスト結果表示</h3>
    <table>
      <thead>
      <tr>
        <th>No</th>
        <th>Item</th>
        <th>Result</th>
      </tr>
      </thead>
      <tbody>
      <tr v-for="result in results" :key="result.No">
        <td>{{ result.No }}</td>
        <td>{{ result.Item }}</td>
        <td>{{ result.Result }}</td>
      </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'ResultTable',
  props: {
    results: {
      type: Array,
      default: () => [],
    },
  },
});
</script>

<style scoped>
table {
  width: 100%;
  border-collapse: collapse;
  margin-top: 10px;
}

th,
td {
  border: 1px solid #ddd;
  padding: 8px;
  text-align: left;
}

th {
  background-color: #f0f0f0;
}
</style>