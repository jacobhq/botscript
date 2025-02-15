import type { TuonoConfig } from 'tuono/config'
import tailwindcss from '@tailwindcss/vite'
import wasm from "vite-plugin-wasm";

const config: TuonoConfig = {
  vite: {
    plugins: [
      tailwindcss(),
      wasm()
    ],
  },
}

export default config
