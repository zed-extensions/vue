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
