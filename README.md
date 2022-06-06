Swirl
=====

<p align=center>
  <img src ="./web/public/logo.svg" style="height: 80px">
</p>

A simple read-only web frontend for [Soft Serve](https://github.com/charmbracelet/soft-serve). 🌀

## Features
- Browse repos, files, and commits
- Preview and download files


## About
I made this project to learn about Next.js and building API's with Rust. Feel free to open an issue if you have a feature requests or found a bug.

You can see it in action on [swirl.stssrn.dev](https://swirl.stssrn.dev).

Swirl currently only shows repos that are listed and marked as public in your Soft Serve config.

## Setup
This project exists out of two parts; a frontend written in Next.js, and an API backend written in Rust which it communicates with.

### Frontend
First, you have to setup the two environment variables:
```
// The location of Swirl's backend
API_HOST=http://localhost:34342

// the location of your Soft Serve server
SOFT_HOST=ssh://localhost:23231
```

You can either deploy the frontend on a service like [Cloudflare Pages](https://pages.cloudflare.com/) or run it on your own hardware.

Running the development server:
```bash
npm run dev
```

Building and running the production server:
```bash
npm run build
npm run start
```

### Backend
First, you have to set up some environment variables. Displayed are the defaults:
```
// Soft Serve's "Home" repo. The readme of this file is used as Swirl's home page.
SOFT_SERVE_HOME_REPO=config
// Path where Soft Serve stores repos
SOFT_SERVE_REPO_PATH=.repos
// Port you want the server to run on
SWIRL_PORT=34342
// Hosts that are allowed to use the API. You could use the host of
// the frontend if you only want your frontend to have access to the API
SWIRL_ALLOWED_ORIGINS=*
// Log level (warn, info, debug, trace)
RUST_LOG=""
```

Building the server:
```bash
cargo build --release
```
You can find the executable in the `target/release` directory.
