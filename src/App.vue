<script setup lang="ts">
import { ref } from 'vue';
import ByteInput from '@/components/ByteInput.vue';
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectTrigger,
    SelectValue,
  } from '@/components/ui/select'

import {
    Table,
    TableBody,
    TableCaption,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
  } from '@/components/ui/table'

const value = ref<string>('65');
</script>

<template>
<div class="p-5 flex flex-col gap-5">
    <div class="flex gap-5">
        <Input/>
        <Select>
            <SelectTrigger>
                <SelectValue placeholder="Select a fruit" />
            </SelectTrigger>
            <SelectContent>
                <SelectGroup>
                    <SelectLabel>Fruits</SelectLabel>
                    <SelectItem value="apple">
                        Apple
                    </SelectItem>
                </SelectGroup>
            </SelectContent>
        </Select>
        <Button>Connect</Button>
    </div>
    <Table>
        <TableCaption>A list of your recent invoices.</TableCaption>
        <TableHeader>
          <TableRow>
            <TableHead>
              Sequence Name
            </TableHead>
            <TableHead>Byte Sequence</TableHead>
            <TableHead></TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow>
            <TableCell class="font-medium">
              Testing
            </TableCell>
            <TableCell>0xFF 0xA3 0x00</TableCell>
            <TableCell class="justify-end flex gap-5">
                <Button variant="destructive">Delete</Button>
                <Button variant="secondary">Edit</Button>
                <Button>Send</Button>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
      <Separator />
    <div class="p-6">
        <ByteInput v-model="value" />
    </div>
</div>
</template>
