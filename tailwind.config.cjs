/** @type {import('tailwindcss').Config}*/
const config = {
	content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],
  
	plugins: [require('flowbite/plugin')],
  
	darkMode: 'class',
  
	theme: {
	  extend: {
		colors: {
		},
		fontFamily: {
			'sans': "AROneSans",
		},
	  },
	}
  };
  
  module.exports = config;
