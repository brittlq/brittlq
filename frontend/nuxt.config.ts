import { defineNuxtConfig } from 'nuxt'

// https://v3.nuxtjs.org/api/configuration/nuxt.config
export default defineNuxtConfig({
    modules: ['@pinia/nuxt', '@nuxtjs/tailwindcss'],
    typescript: {
        typeCheck: true
    }
})
