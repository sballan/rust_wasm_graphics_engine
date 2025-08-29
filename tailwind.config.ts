import type { Config } from "tailwindcss";

export default {
  content: [
    "web/{routes,islands,components}/**/*.{ts,tsx}",
  ],
} satisfies Config;