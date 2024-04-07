/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  safelist: [
    "bg-black",
    "bg-white"
  ],
  content: [
        // include all rust, html and css files in the src directory
        "./src/**/*.{rs,html,css}",
        // include all html files in the output (dist) directory
        "./dist/**/*.html", 
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}

