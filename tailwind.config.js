module.exports = {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx,svelte}'],
  theme: {
    extend: {
      animation: {
        blob: 'blob 10s infinite',
      },
      keyframes: {
        blob: {
          '0%': { transform: 'translate(0%, 0%) scale(1)' },
          '33%': { transform: 'translate(30%, -50%) scale(1.25)' },
          '66%': { transform: 'translate(-15%, 10%) scale(0.75)' },
          '100%': { transform: 'translate(0%, 0%) scale(1)' },
        },
      },
    },
  },
  plugins: [],
};
