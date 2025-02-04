// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },

  modules: ["@vesp/nuxt-fontawesome", "@pinia/nuxt", "@nuxt/ui"],
  colorMode: {
    classSuffix: '',
  },
  fontawesome: {
    component: 'fa',
    icons: {
      solid: ["sun", "moon", "eye", "eye-slash"],
      regular: [],
      brands: [],
    }
  }
})
