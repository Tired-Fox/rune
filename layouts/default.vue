<template>
  <div class="flex flex-col w-full h-[100vh]">
    <div
      class="w-full flex justify-between px-6 py-4"
    >
      <div>
        <nuxt-link
          class="px-2 py-1 border rounded-md"
          to="/"
        >
          Home
        </nuxt-link>
      </div>
      <div>
        <button
          @click="() => {
            if (theme.preference === 'light') {
              theme.preference = 'dark';
            } else if (theme.preference === 'dark') {
              theme.preference = 'system'
            } else {
              theme.preference = 'light';
            }
          }"
        >
          <fa :icon="theme.value === 'light' ? ['fas', 'sun'] : ['fas', 'moon']" />
          {{ theme.preference }}
        </button>
      </div>
    </div>
    <slot />
  </div>
</template>

<script setup lang="ts">
const theme = useColorMode();

const accountStore = useAccountStore();
const { account } = storeToRefs(accountStore);
</script>
