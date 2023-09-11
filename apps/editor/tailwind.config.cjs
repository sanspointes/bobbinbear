/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./src/**/*.{js,jsx,ts,tsx}'],
    theme: {
        extend: {
            colors: {
                orange: {
                    50: '#fff9ec',
                    100: '#fff1d2',
                    200: '#ffdfa4',
                    300: '#ffc86b',
                    400: '#ffa42f',
                    500: '#ff8707',
                    600: '#f96a00',
                    700: '#de5500',
                    800: '#a33e09',
                    900: '#83350b',
                    950: '#471803',
                },
            },
        },
    },
    plugins: [require('@kobalte/tailwindcss')({ prefix: 'kb' })],
};
