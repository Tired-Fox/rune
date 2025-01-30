import tailwindcss from '@tailwindcss/vite';

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },
  css: ["~/assets/css/main.css"],
  vite: {
    plugins: [
      tailwindcss()
    ]
  },

  modules: ["@nuxtjs/color-mode", "@vesp/nuxt-fontawesome", "@pinia/nuxt"],
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
