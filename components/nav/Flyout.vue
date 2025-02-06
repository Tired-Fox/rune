<template>
  <Sheet
    v-model:open="open"
    side="left"
    :transition="false"
    :overlay="false"
  >
    <SheetTrigger as-child>
      <Button class="w-fit sticky top-2 left-2 z-20 bg-amber-400 hover:bg-amber-300 dark:bg-amber-500 dark:hover:bg-amber-400" @click="open = true">
        <Menu class="w-4 h-4" />
      </Button>
    </SheetTrigger>
    <SheetContent side="left">
      <SheetHeader class="flex flex-row gap-2 pt-4">
        <Button
          class="w-fit bg-amber-400 hover:bg-amber-300 dark:bg-amber-500 dark:hover:bg-amber-400"
          @click="navigateTo('/')"
        >
          <Icon name="i-heroicons-home" class="w-5 h-5" />
        </Button>
        <span class="relative grow">
          <Input
            ref="searchInput"
            icon="i-heroicons-magnifying-glass-20-solid"
            class="w-full"
          />
          <small class="flex gap-1 absolute right-2 bottom-2.5 text-xs">
            Ctrl + K
          </small>
        </span>
        <Button variant="outline">
          <Icon name="i-heroicons-funnel" />
        </Button>
      </SheetHeader>

      <SheetFooter>
      </SheetFooter>
    </SheetContent>
  </Sheet>
</template>

<script setup lang="ts">
import { Menu } from 'lucide-vue-next';
import { useMagicKeys, whenever } from '@vueuse/core';

const searchInput = ref<HTMLInputElement|null>(null);
const open = ref(false);

const contentRating = reactive({
  "safe": false,
  "suggestive": false,
  "erotic": false,
  "pornographic": false,
});

const { CTRL_K, CTRL_B } = useMagicKeys();
whenever(CTRL_B, () => {
  open.value = open.value ? false : true;
});
whenever(CTRL_K, () => {
  if (!open.value) open.value = true;

  nextTick(() => {
    if (searchInput.value) {
      searchInput.value.target.focus()
    }
  });
});
</script>
