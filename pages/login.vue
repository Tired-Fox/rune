<template>
  <div class="w-full h-full flex flex-col items-center justify-center p-4">
    <form
      class="flex flex-col gap-6 w-full max-w-[70ch] p-10 dark:bg-zinc-800 rounded-md shadow shadow-black"
      @submit="submit"
    >
      <div class="flex flex-col gap-2">
        <div class="flex flex-col gap-1">
          <label>Client Id</label>
          <input
            v-model="account.creds.id"
            type="text"
            name="clientId"
            class="rounded-md px-2 py-1 dark:bg-zinc-700"
            placeholder="client id..."
          />
        </div>
        <div class="flex flex-col gap-1">
          <label>Client Secret</label>
          <div class="relative">
            <input
              :value="secret"
              @input="setSecret"
              :type="secretVisible ? 'text' : 'password'"
              name="clientSecret"
              class="rounded-md pl-2 pr-10 py-1 dark:bg-zinc-700 focus:border dark:border-zinc-700 w-full"
              autocomplete="false"
              placeholder="client secret..."
            />
            <button
              class="absolute right-0 top-0 h-full w-10 flex justify-center items-center cursor-pointer"
              :title="`${secretVisible ? 'Hide' : 'Show'} Client Secret`"
              @click="() => secretVisible = !secretVisible"
            >
              <fa :icon="secretVisible ? ['fas', 'eye'] : ['fas', 'eye-slash']" />
            </button>
          </div>
        </div>
      </div>

      <hr class="w-3/4 mx-auto" />

      <div class="flex flex-col gap-2">
        <div class="flex flex-col gap-1">
          <label>Username</label>
          <input
            v-model="username"
            type="text"
            name="username"
            class="rounded-md px-2 py-1 dark:bg-zinc-700"
            placeholder="username..."
          />
        </div>
        <div class="flex flex-col gap-1">
          <label>Password</label>
          <div class="relative">
            <input
              v-model="password"
              :type="passwordVisible ? 'text' : 'password'"
              class="rounded-md pl-2 pr-10 py-1 dark:bg-zinc-700 focus:border dark:border-zinc-700 w-full"
              autocomplete="false"
              placeholder="password..."
            />
            <button
              class="absolute right-0 top-0 h-full w-10 flex justify-center items-center cursor-pointer"
              :title="`${passwordVisible ? 'Hide' : 'Show'} Password`"
              @click="() => passwordVisible = !passwordVisible"
            >
              <fa :icon="passwordVisible ? ['fas', 'eye'] : ['fas', 'eye-slash']" />
            </button>
          </div>
        </div>
      </div>
      <button
        type="submit"
        class="border rounded-md px-2 py-1 dark:bg-slate-50 dark:text-zinc-900 hover:opacity-95 cursor-pointer disabled:cursor-default dark:disabled:opacity-50 dark:autofill:bg-zinc-700"
        :disabled="!account.creds.id?.length || !account.creds.secret?.length || !password.length || !username.length"
      >
        Log In
      </button>

      <span v-if="error.length" class="mt-4 text-rost-500">{{ error }}</span>
    </form>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

const accountStore = useAccountStore();
const { account } = storeToRefs(accountStore);

const secretVisible = ref(false);
const passwordVisible = ref(false);

const secret = computed(() => atob(account.value.creds.secret ?? ''));
const username = ref("");
const password = ref("");

const error = ref("");

function setSecret(value: string) {
  account.value.creds.secret = btoa(value);
}

function submit(e: Event) {
  e.preventDefault();
  invoke("login", {
    creds: account.value.creds,
    username: username.value,
    password: password.value,
  })
    .then(() => {
      accountStore.fetchAccount()
        .then(() => { navigateTo('/') });
    })
    .catch(e => error.value = e)
}

</script>
