 
import { defineConfig, loadEnv } from 'vite'
import react from '@vitejs/plugin-react'
import { sentryVitePlugin } from "@sentry/vite-plugin"
import svgr from 'vite-plugin-svgr'

const env = loadEnv("", process.cwd())

// https://vite.dev/config/
export default defineConfig({
  build: {
    sourcemap: true
  },
  plugins: [react(), svgr({include: "**/*.svg?react"}), sentryVitePlugin({
    org: env.VITE_SENTRY_ORG,
    project: env.VITE_SENTRY_PROJECT,
    authToken: env.VITE_SENTRY_AUTH_TOKEN,
  })],
  server: {
    proxy: {
      '/api': {
        target: 'https://events.jonsen.se',
        changeOrigin: true,
        secure: true
      }
    }
  }
})
