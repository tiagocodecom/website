/** @type {import('tailwindcss').Config} */

module.exports = {
  content: {
    files: ['*.html', './src/**/*.rs'],
    transform: {
      rs: (content) => content.replace(/(?:^|\s)class:/g, ' ')
    }
  },
  darkMode: 'selector',
  theme: {
    extend: {
      fontFamily: {
        "opensans": ['Open Sans', 'sans-serif'],
        "poppins": ['Poppins', 'sans-serif'],
        "mono": ['Roboto Mono', 'monospace'],
      },
      boxShadow: {
        'sectionBoxShadow': '0 8px 26px 0 rgba(22,24,26,0.07)',
        'sectionBoxShadowHover': '0 8px 32px 0 rgba(22,24,26,0.11)',
        'avatarText': '0 8px 26px 0 rgba(22,24,26,0.11)',
        'darkBox': '0 8px 26px 0 rgba(0, 0, 0, 0.3)',
      },
      colors: {
        'main': '#ebf4fa',
        'boxDark': '#1c1e20',
        'mainfont': '#242424',
      },
      transitionTimingFunction: {
        'custom': 'cubic-bezier(0.165, 0.84, 0.44, 1)',
      },
      keyframes: {
        lineMove: {
          '0%': {
            top: '-80px',
          },
          '100%': {
            top: '100%',
          }
        },
        loaderLetter: {
          '0%': {
            opacity: '1',
          },
          '100%': {
            opacity: '0',
          }
        },
      },
      animation: {
        bgLine: 'lineMove 8s linear infinite',
        loader: 'loaderLetter 1.0s linear infinite',
      },
    },
  },
  plugins: [],
}