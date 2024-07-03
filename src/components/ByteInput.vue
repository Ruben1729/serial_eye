<script setup lang="ts">
import {Input} from '@/components/ui/input';
import {useVModel} from '@vueuse/core';
import {ByteFormat, FORMATS} from "@/lib/byte_format.ts";
import {ref} from "vue";
import {Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select";

const props = defineProps<{
    modelValue: string;
    disabled?: boolean;
}>();
const emit = defineEmits(['update:modelValue']);
const inputValue = useVModel(props, 'modelValue', emit);
const format = ref<ByteFormat>(ByteFormat.DECIMAL);

function parseValue(value: string, format: ByteFormat): number {
    const config = FORMATS[format];
    return config.parse(value);
}

function stringValue(value: number, format: ByteFormat): string {
    const config = FORMATS[format];
    return config.string(value);
}

function handleChange(e: Event) {
    const input = e.target as HTMLInputElement;
    const config = FORMATS[format.value];
    inputValue.value = input.value.replaceAll(config.regex, '').toUpperCase();
}

function handleChangeFormat(value: ByteFormat) {
    const n = parseValue(inputValue.value, format.value);
    let str = stringValue(n, value);
    if (Number.isNaN(n)) {
        str = '';
    }
    inputValue.value = str;
    format.value = value;
}
</script>

<template>
    <div class="flex gap-2">
        <Select :modelValue="format" @update:modelValue="handleChangeFormat">
            <SelectTrigger class="w-[200px]">
                <SelectValue />
            </SelectTrigger>
            <SelectContent>
                <SelectGroup>
                    <SelectItem v-for="(config, key) in FORMATS" :key="key" :value="key">
                        {{ config.name }}
                    </SelectItem>
                </SelectGroup>
            </SelectContent>
        </Select>
        <Input v-model="inputValue" :disabled="props.disabled" @input="handleChange" />
    </div>
</template>
