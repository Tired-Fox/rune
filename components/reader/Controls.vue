<template>
  <div ref="root" class="absolute top-0 left-0 bottom-0 right-0 z-10 select-none">
    <div class="absolute left-0 bottom-0 top-0 right-1/2 w-1/2 text-transparent" @click="prevPage">
      .
    </div>
    <div class="absolute left-1/2 bottom-0 top-0 right-0 text-transparent" @click="nextPage">
      .
    </div>
  </div>
  <div class="fixed h-0 left-0 right-0 bottom-6 z-20 px-2 flex justify-between text-xs text-gray-800">
    <div v-if="zoomControls" class="w-fit flex items-center py-0 shadow shadow-black/20 dark:shadow-black rounded-full">
      <button @click="zoomOut" class="px-2 py-1 bg-amber-400 dark:bg-amber-500 hover:bg-amber-300 dark:hover:bg-amber-400 rounded-l-full">
        <ChevronLeft class="w-3" />
      </button>
      <button class="px-2 bg-amber-400 dark:bg-amber-500 hover:bg-amber-300 dark:hover:bg-amber-400 h-8 w-full grid items-center" @click="resetZoom">
        {{ Math.round(zoom * 100) }}%
      </button>
      <button @click="zoomIn" class="px-2 py-1 bg-amber-400 dark:bg-amber-500 hover:bg-amber-300 dark:hover:bg-amber-400 hover:bg-amber-300 dark:hover:bg-amber-400 rounded-r-full">
        <ChevronRight class="w-3" />
      </button>
    </div>
    <div v-if="pageControls" class="w-fit flex items-center py-0 shadow shadow-black/20 dark:shadow-black rounded-full text-gray-800">
      <button @click="prevPage" class="px-2 py-1 bg-amber-400 dark:bg-amber-500 hover:bg-amber-300 dark:hover:bg-amber-400 rounded-l-full">
        <ChevronLeft class="w-3" />
      </button>
      <button class="px-2 bg-amber-400 dark:bg-amber-500 hover:bg-amber-300 dark:hover:bg-amber-400 h-8 w-full grid items-center" @click="resetPage">
        {{ page + 1 }} / {{ count }}
      </button>
      <button @click="nextPage" class="px-2 py-1 bg-amber-400 dark:bg-amber-500 hover:bg-amber-300 dark:hover:bg-amber-400 rounded-r-full">
        <ChevronRight class="w-3" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ChevronLeft, ChevronRight } from 'lucide-vue-next';

const props = defineProps<{
  count: number,
  page: number,

  zoom: number,

  pageControls?: boolean,
  zoomControls?: boolean,
}>();

const emit = defineEmits<{
  'update:zoom': [value: number],
  'update:page': [value: number],

  'zoomIn': [value: number],
  'zoomOut': [value: number],
  'nextPage': [value: number],
  'prevPage': [value: number],
  'resetPage': [value: number],
}>()

const zoomIn = () => emit('zoomIn', props.zoom);
const zoomOut = () => emit('zoomOut', props.zoom);
const nextPage = () => emit('nextPage', props.page);
const prevPage = () => emit('prevPage', props.page);
const resetPage = () => emit('resetPage', props.page);
const resetZoom = () => emit('update:zoom', props.pageControls ? 1 : 0.7);

onMounted(() => nextTick(() => emit('update:zoom', props.pageControls ? 1 : 0.7)));
</script>
