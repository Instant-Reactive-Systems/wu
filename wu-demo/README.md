# `wu` — web utilities built with leptos and tailwind

## Usage
This crate is not on crates.io, so you will have to include it in projects as a git submodule (preferably).

After `wu` is included as a submodule, include it in your `tailwind.config.js` like: `import { plugins, presets } from './wu/wu-tw/index';`. After that, include it in your `Cargo.toml` using the `path` syntax like: `wu = { path = "./wu" }`.

In order to get all the CSS classes from `wu`, you will also need to setup the path to the Rust files on the `tailwind.config.js` in order for tailwind to include those classes as well.

```js
content: {
	files: ["*.html", "./src/**/*.rs", "./wu/src/**/*.rs"],
}
```

After that, everything is set up and the library is ready to be used as normal.

## Contributing
To enable the demo, set the `demo` feature locally.

When adding new components, alongside the component, add examples as a special route and add it to the demo to showcase the functionality.
