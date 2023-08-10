/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        yellow: {
          "50": "#fffeea",
          "100": "#fffbc5",
          "200": "#fff786",
          "300": "#ffed47",
          "400": "#ffde1d",
          "500": "#f5b902",
          "600": "#e09400",
          "700": "#ba6903",
          "800": "#96500a",
          "900": "#7c420b",
          "950": "#472201",
        },
      },
    },
  },
  plugins: [
    require("@kobalte/tailwindcss")({ prefix: "kb" }),
  ],
};
