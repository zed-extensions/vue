# Zed Vue

A [Vue](https://vuejs.org/) extension for [Zed](https://zed.dev).

## Development

To develop this extension, see the [Developing Extensions](https://zed.dev/docs/extensions/developing-extensions) section of the Zed docs.

## Initialization Options

### Specifying location of TypeScript SDK

By default, this extension assumes that you are working in a project with a `node_modules` directory, and searches for
the TypeScript SDK inside that directory.

This may not always be true; for example, when working in a project that uses Yarn PnP, there is no `node_modules`. For
editor support, the [documented](https://yarnpkg.com/getting-started/editor-sdks) approach is to run something like
`yarn dlx @yarnpkg/sdks`. In that case, you can provide the following initialization options in your Zed settings:

```json
{
  "lsp": {
    "vue": {
      "initialization_options": {
        "typescript": {
          "tsdk": ".yarn/sdks/typescript/lib"
        }
      }
    }
  }
}
```

## Settings Options

`lsp.vue.settings` is passed through to the Vue language server (Volar / [`vuejs/language-tools`](https://github.com/vuejs/language-tools)). The following settings are enabled by default:

```jsonc
{
  "lsp": {
    "vue": {
      "settings": {
        // Display inlay hints for the `$event` parameter in inline event handlers.
        "vue.inlayHints.inlineHandlerLeading": true,
        // Display hints when required component props are missing in templates.
        "vue.inlayHints.missingProps": true,
        // Display inlay hints for patterns that wrap component options.
        "vue.inlayHints.optionsWrapper": true,
        // Display inlay hints related to `v-bind` shorthand (`:`).
        "vue.inlayHints.vBindShorthand": true,
      },
    },
  },
}
```

You can find the upstream settings configuration schema [`here`](https://github.com/vuejs/language-tools/blob/ee5041d27940cf6f9a5150635d3b13140a9dff54/extensions/vscode/package.json#L252).

> [!NOTE]: Some settings (e.g. `vue.editor.focusMode`) may not take effect.
