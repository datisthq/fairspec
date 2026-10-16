# Fairspec TypeScript

This guide will help you get started with Fairspec TypeScript. If you are new to the core framework's tecnhologies, please take a look at the [Fairspec standard](https://fairspec.org/) and [Polars DataFrames](https://pola.rs/) documentation.

## Runtimes

> [!TIP]
>
> - It is possible to use Fairspec TypeScript in [Jupyter Notebooks](/typescript/jupyter)!

Fairspec TypeScript and all its packages support all the prominent TypeScript runtimes:

- **Node v20+**
- **Deno v2+**
- **Bun v1+**

The core package `@fairspec/metadata` additionally supports browser environments:

- **Edge v92+**
- **Chrome v92+**
- **Firefox v90+**
- and others

## Installation

> [!NOTE]
> The documentation uses `npm` command to install packages. If you are using other package managers, please adjust the commands accordingly.

The framework can be installed as one package:

```bash
npm install fairspec
```

You car cherry-pick from individual packages:

```bash
npm install @fairspec/metadata @fairspec/table
```

In the browser, the core package can be just imported using NPM CDNs:

```js
import { loadDatasetDescriptor } from "https://esm.sh/@fairspec/metadata"
```

## TypeScript

> [!TIP]
> Use **Node v24+** to be able to run TypeScript files directly with the `node` binary like `node my-data-script.ts`

Fairspec TypeScript is built with type safety in mind. It uses TypeScript to provide type definitions for all packages and to enforce type safety throughout the framework. It's highly reccomended to setup a TypeScript aware environment to work with the project.

## Examples

Loading a Dataset from Zenodo merging system Zenodo metadata into a user dataset and validating its metadata:

```ts
import { loadDataset } from "fairspec"

const { dataset } = await loadDataset("https://zenodo.org/records/10053903")

console.log(dataset)
//{
//  id: 'https://doi.org/10.5281/zenodo.10053903',
//  ...
//}
```

Validating an in-memory dataset descriptor:

```ts
import { validateDatasetDescriptor } from "fairspec"

const report = await validateDatasetDescriptor({ resources: "bad" })

console.log(report.valid)
// false
console.log(report.errors)
//[
//  {
//    type: "metadata",
//    message: "must have type array",
//    jsonPointer: '/resources',
//  }
//]
```

Loading a dataset from a remote descriptor and saving it locally as a zip archive, and then using it as a local dataset:

```ts
import {
  loadDatasetDescriptor,
  loadDatasetFromZip,
  saveDatasetToZip,
  getTempFilePath,
} from "fairspec"

const archivePath = getTempFilePath()
const sourcePath = await loadDatasetDescriptor(
  "https://raw.githubusercontent.com/roll/currency-codes/refs/heads/master/datapackage.json",
)

await saveDatasetToZip(sourceDataset, { archivePath })
const targetDataset = await loadDatasetFromZip(archivePath)
console.log(targetDataset)
```

Reading a CSV table:

```ts
import { loadTable } from "fairspec"

const table = await loadTable({ data: "data.csv" })

// Load with custom format
const table = await loadTable({
  path: "data.csv",
  format: {
    name: "csv",
    delimiter: ";",
    headerRows: false,
  },
})
```

## Reference

See **Reference** of each individual package for more details. Note, that `fairspec` and `@fairspec/library` packages re-export most of the functionality.
