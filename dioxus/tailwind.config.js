/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
      // include all rust, html and css files in the src directory
      "./src/**/*.{rs,html,css}",
      // include all html files in the output (dist) directory
      "../docs/**/*.html",
  ],
  theme: {
    latex: {
			footnotes: false,
			syntax: true,
		},
    extend: {},
  },
  plugins: [require("latex-tailwind")],
}

