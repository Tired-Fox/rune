<template>
  <div class="sticky top-0 bg-background w-full h-fit py-2 px-4 shadow z-10">
    <NavigationMenu class="!p-0">
      <NavigationMenuList>
        <NavigationMenuItem>
          <Button @click="navigateTo('/')" class="px-3 py-0 bg-amber-400 hover:bg-amber-300">
            <Icon name="i-heroicons-home" class="w-4 h-4" />
          </Button>
        </NavigationMenuItem>
        <NavigationMenuItem>
          <div class="relative w-fit ml-2">
            <Input ref="searchInput" type="text" class="pl-7" @keydown="(e: Event) => e.stopPropagation()" />
            <Icon name="i-heroicons-magnifying-glass" class="absolute top-2.5 left-2" />
          </div>
        </NavigationMenuItem>
        <NavigationMenuItem>
          <Button
            variant="outline"
          >
            <Icon name="i-heroicons-funnel" />
          </Button>
        </NavigationMenuItem>
      </NavigationMenuList>
    </NavigationMenu>
  </div>
</template>

<script setup lang="ts">
import { useMagicKeys, whenever } from '@vueuse/core';

const searchInput = ref<HTMLInputElement|null>(null);

const { CTRL_K } = useMagicKeys();
whenever(CTRL_K, () => {
  nextTick(() => {
    if (searchInput.value) {
      searchInput.value.target?.focus()
    }
  });
});
</script>
