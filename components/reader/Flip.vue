<template>
  <div class="relative w-full h-full pt-5">
    <div
      class="w-[calc(100vw-4rem)] h-[calc(100vh-7rem)] flex justify-center items-center"
      :style="{
        zoom: zoom
      }"
    >
      <Skeleton v-if="!pages.length" class="w-full h-full max-w-[800px]  aspect-[2/3] rounded" />
      <div
        v-for="(path, i) in pages"
        :key="path"
        class="w-[calc(100vw-4rem)] h-[calc(100vh-7rem)] rounded border border-rose-500"
        :class="{ 'w-0 h-0 overflow-hidden absolute': i !== page }"
        :style="{
          backgroundImage: `url('${path}')`,
          backgroundSize: i === page ? 'contain' : '0',
          backgroundRepeat: 'no-repeat',
          backgroundPosition: 'center',
        }"
      >
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  zoom: number,
  page: number,
  pages: string[]
}>()
</script>
