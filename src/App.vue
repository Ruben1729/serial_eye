<script setup lang="ts">
import { ref } from 'vue';
import ByteInput from '@/components/ByteInput.vue';

const value = ref<string>('65');
</script>

<template>
    <div class="p-6">
        <ByteInput v-model="value" />
    </div>
</template>
