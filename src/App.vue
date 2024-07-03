<script setup lang="ts">
import { ref } from 'vue';
import ByteInput from '@/components/ByteInput.vue';
import { ByteFormat } from '@/lib/byte_format.ts';

const value = ref<number>(65);
const byteFormat = ref<ByteFormat>(ByteFormat.HEX);
</script>

<template>
  <ByteInput v-model="value" :format="byteFormat" />
</template>
