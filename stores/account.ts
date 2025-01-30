import { invoke } from "@tauri-apps/api/core";

export type Creds = {
  id?: string,
  /** Base64 Encoded */
  secret?: string,
};

export type Account = {
  loggedIn: boolean,
  creds: Creds,
};

export const useAccountStore = defineStore('account', () => {
  const account: Ref<Account> = ref({
    loggedIn: false,
    creds: {}
  });

  async function fetchAccount() {
    return invoke<Account>('fetch_account', {})
      .then(response => {
        console.log(response);
        account.value = response;
      })
      .catch(response => {
        console.error(response);
      })
  }

  onMounted(() => {
    invoke<Account>('fetch_account', {})
      .then(response => {
        console.log(response);
        account.value = response;
      })
      .catch(response => {
        console.error(response);
      })
      .finally(() => {
        if (!account.value.loggedIn) {
          navigateTo('/login')
        }
      })
  })

  return {
    account,
    
    fetchAccount,
  }
});
