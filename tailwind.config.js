/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./index.html', './src/**/*.{vue,js}'],
  plugins: [
      require('@tailwindcss/typography')
  ],
};
