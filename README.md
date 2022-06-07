Swirl
=====

<p>
  <img src ="./web/public/logo.svg" style="height: 80px">
</p>

A simple read-only web frontend for [Soft Serve](https://github.com/charmbracelet/soft-serve). 🌀

## Features
- Browse repos, files, and commits
- Preview and download files


## About
I made this project to learn about Next.js and building APIs with Rust. Feel free to open an issue if you have a feature requests or found a bug.

You can see it in action on [swirl.stssrn.dev](https://swirl.stssrn.dev).

Swirl currently only shows repos that are listed and marked as public in your Soft Serve config.

## Setup
This project exists out of two parts:
- A frontend written in Next.js
- An API backend written in Rust.

### Frontend
First, you have to setup the two environment variables:
```
// The location of Swirl's backend
API_HOST=http://localhost:34342

// the location of your Soft Serve server
SOFT_HOST=ssh://localhost:23231
```

You can either deploy the frontend on a service like [Vercel](https://vercel.com/) or run it on your own hardware.

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

## Known issues
- SVG previews are broken
- Only git summaries are displayed
- Git diffs aren't supported yet
- <p>Due to me making a false assumption about Next.js, I decided to client-side render all pages. I thought that I'd be able to render all the pages to static HTML files. In this case it doesn't seem to be possible, because <code>next export</code> causes routing to break.</p> <p>Since that didn't work out, I'm considering either letting Next.js render pages on the server-side, or cutting out Next.js entirely and rendering pages on the Rust backend, making the API layer unnecessary.</P>
