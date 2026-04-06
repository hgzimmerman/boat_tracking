/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  safelist: [
    "bg-black",
    "bg-white",
    "bg-green-500",
    "bg-red-500",
    "bg-yellow-500",
    "bg-blue-500",
    "min-w-64",
    "gap-3",
    "shadow-lg"
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

