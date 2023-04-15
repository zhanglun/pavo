module.exports = {
  darkMode: 'class',
  content: [
    './src/**/*.rs',
  ],
  theme: {
    colors: ({ colors }) => {
      return {
        ...colors,
        medirian: {
          '1': '#283c86',
          '2': '#45a247'
        },
        'bayOfMany': {
          '50': '#f0f5fe',
          '100': '#dee9fb',
          '200': '#c4daf9',
          '300': '#9bc1f5',
          '400': '#6ca0ee',
          '500': '#4a7fe7',
          '600': '#3561db',
          '700': '#2c4ec9',
          '800': '#2a40a3',
          '900': '#283c86',
        },
        'apple': {
          '50': '#f3faf3',
          '100': '#e3f5e3',
          '200': '#c8eac8',
          '300': '#9dd89e',
          '400': '#6abe6c',
          '500': '#45a247',
          '600': '#348536',
          '700': '#2c692e',
          '800': '#275428',
          '900': '#224524',
        },
      }
    },
    extend: {
      backgroundImage: (theme) => ({
        'button-gradient': `linear-gradient(to right, ${theme('colors.bayOfMany.900')} 0%, ${theme('colors.apple.500')}  51%, ${theme('colors.bayOfMany.900')} 100%)`,
      }),
    },
  },
  plugins: [
  ]
}
