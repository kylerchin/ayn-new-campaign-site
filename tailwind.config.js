/** @type {import('tailwindcss').Config} */
    module.exports = {
      content: {
        relative: true,
        files: ["*.html", "./src/**/*.rs"],
      },
      theme: {
        extend: {
          colors: {
            "octablue": "#539BED",
            "octaorange": "#FAA638"
          }
        },
      },
      plugins: [],
    }
    