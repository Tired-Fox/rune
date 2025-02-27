// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  devtools: { enabled: true },

  modules: ["@vesp/nuxt-fontawesome", "@pinia/nuxt", "@nuxtjs/tailwindcss", "@nuxtjs/color-mode", "shadcn-nuxt", "@nuxt/icon"],
  colorMode: {
    classSuffix: '',
  },
  shadcn: {
    prefix: '',
    componentDir: './components/ui'
  },
  tailwindcss: {
    config: {
      safelist: [
        'hidden',
        'w-0',
        'h-0',
        'overflow-hidden'
      ]
    }
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
