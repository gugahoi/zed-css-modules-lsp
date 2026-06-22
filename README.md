# Zed CSS Modules extension

A [Zed](https://zed.dev) extension that wires up
[`cssmodules-language-server`](https://github.com/antonk52/cssmodules-language-server)
so you get **Go To Definition**, hover, and completion from your JS/TS files straight into
the matching `*.module.css` file.

Put the cursor on a class in `styles.someClass` and "Go to Definition" jumps to the
`.someClass { … }` rule in the CSS module.

## What it does

The extension attaches the language server to Zed's built-in **TSX**, **TypeScript**, and
**JavaScript** languages. The server follows your `import styles from './Foo.module.css'`
statements and resolves class names against the stylesheet (CSS / PostCSS / Sass / SCSS /
Stylus).

## Install (local dev extension)

1. Install the wasm target once:
   ```sh
   rustup target add wasm32-wasip1
   ```
2. In Zed, open the command palette and run **`zed: install dev extension`**, then select
   this folder. Zed compiles the crate to wasm and loads it.
3. After editing the extension, re-run **`zed: install dev extension`** to reload.

The language server binary is obtained automatically: the extension first looks for
`cssmodules-language-server` on your `$PATH`, and if it isn't found, downloads the npm
package using Zed's bundled Node — no manual setup required.

## Configuration

Set initialization options under `lsp.cssmodules` in your Zed `settings.json`. The
`camelCase` option controls how kebab-case classes appear in completions
(`true` | `"dashes"` | `false`):

```json
{
  "lsp": {
    "cssmodules": {
      "initialization_options": {
        "camelCase": "dashes"
      }
    }
  }
}
```

## Go To Definition vs. TypeScript

Zed's default TypeScript server (`vtsls`) also answers "Go to Definition". For the class
token in `styles.foo`, `vtsls` usually returns nothing (the property has no source
location), so the CSS module result is the only one and you land directly in the stylesheet.

If you use [`typescript-plugin-css-modules`](https://github.com/mrmckeb/typescript-plugin-css-modules)
(which generates real types), `vtsls` will also resolve and Zed may show multiple results.
You can prioritize this server per language in `settings.json`:

```json
{
  "languages": {
    "TSX": { "language_servers": ["cssmodules", "..."] },
    "TypeScript": { "language_servers": ["cssmodules", "..."] }
  }
}
```

Hard-disabling vtsls (`"!vtsls"`) would suppress the TS result but also disables normal
TypeScript navigation, so it isn't recommended. As an alternative, the server also answers
**Go to Implementation**, which never collides with TypeScript's definition.
