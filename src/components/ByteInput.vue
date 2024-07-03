<template>
  <Input v-model="inputValue" @input="updateValue" />
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { Input } from '@/components/ui/input';
import { ByteFormat } from '@/lib/byte_format.ts';

const props = defineProps<{
  format: ByteFormat;
  modelValue: string;
}>();

const emit = defineEmits(['update:modelValue']);
const inputValue = ref<string>('');

watch(() => props.modelValue, (newValue) => {
  inputValue.value = formatValue(newValue, props.format);
}, { immediate: true });

function formatValue(value: string | number, format: ByteFormat): string {
  switch (format) {
    case ByteFormat.ASCII:
      return typeof value === 'number' ? String.fromCharCode(value) : value;
    case ByteFormat.HEX:
      return typeof value === 'number' ? value.toString(16) : parseInt(value).toString(16);
    case ByteFormat.BINARY:
      return typeof value === 'number' ? value.toString(2) : parseInt(value).toString(2);
    case ByteFormat.DECIMAL:
      return value.toString();
    default:
      return value.toString();
  }
}

function parseValue(value: string, format: ByteFormat): string {
  switch (format) {
    case ByteFormat.ASCII:
      return value.charCodeAt(0);
    case ByteFormat.HEX:
      return parseInt(value, 16);
    case ByteFormat.BINARY:
      return parseInt(value, 2);
    case ByteFormat.DECIMAL:
      return parseInt(value, 10);
    default:
      return value;
  }
}

function updateValue(event: Event) {
  const target = event.target as HTMLInputElement;
  const newValue = target.value;
  emit('update:modelValue', parseValue(newValue, props.format));
}
</script>
