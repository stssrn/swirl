const path = require('path')

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  images: {
    domains: ['[::1]'],
  },

  sassOptions: {
    includePaths: [path.join(__dirname, 'styles')],
    prependData: '@import "variables.scss";'
  }
}

module.exports = nextConfig
