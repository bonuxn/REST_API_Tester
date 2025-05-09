<template>
  <div>
    <label v-if="label">{{ label }}</label>
    <input
        :type="type"
        :value="modelValue"
        @input="$emit('update:modelValue', $event.target.value)"
    />
  </div>
</template>

<script>
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'InputField',
  props: {
    label: String,
    type: {
      type: String,
      default: 'text',
    },
    modelValue: [String, Number],
  },
  emits: ['update:modelValue'],
});
</script>

<style scoped>
label {
  display: block;
  margin-bottom: 5px;
}

input {
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  width: 300px; /* デフォルトの幅 */
}
</style>