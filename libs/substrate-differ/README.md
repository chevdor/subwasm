# Substrate Differ

## Overview

This crate helps comparing/diffing 2 version of Substrate metadata or 2 Runtimes. A basic solution consists in serializing the metadata as a json string and comparing visually the ....16534 lines you will see. That is just not serioulsy doable.

## Context

The Substrate metadata format is versioned. For a given version however, the content may change between runtime versions. Whether the metadata change or not is critical. Adding new modules or calls has usually a low impact. However, removing, modifying or reoredring modules and/or their content is critical to the various client APIs.

## Comparison of diff methods

This crate offers 3 methods to compare metadata, they have benefits and drawbacks, as shown in the table below.

|              | raw_differ | partial_differ | summary_differ | Comments                                                       |
| ------------ | ---------- | -------------- | -------------- | -------------------------------------------------------------- |
| comparing    | metadata   | metadata       | runtime        |                                                                |
| verbosity    | HIGH       | MEDIUM         | LOW            | How long is the output                                         |
| accuracy     | HIGH       | LOW            | HIGH           | How well it describes changes                                  |
| genericity   | HIGH       | NONE           | HIGH           | How good it works for past and future Metadata formats/version |
| detail level | HIGH       | MEDIUM         | LOW            | How good we can spot where the differences come from           |
| future proof | YES        | NO             | YES            |                                                                |

### `summary_differ`

The `summary_differ` works on a very high level, it compares and show aggregated information such as the total size and whether the object are strictly identical or not. Here we are diffing **runtimes**.

The result looks similar to:

```
Running subwasm v0.8.0
  🅰️  File("runtime_a.wasm")
  🅱️  File("runtime_b.wasm")
  🅰️  1.916 MB (2,008,974 bytes)
  🅱️  2.795 MB (2,930,552 bytes)
Checking metadata versions:
  ✅ Both metadata versions are identical: V12
Checking core versions:
  ❌ The 2 core versions are different:
  🅰️  kusama-2030 (parity-kusama-0.tx5.au2)
  🅱️  kusama-9000 (parity-kusama-0.tx5.au2)
Checking runtime metadata:
  ❌  The metadata are different
```

This is a good option to confirm that the metadata between 2 runtimes did NOT change. Note that 2 runtimes may have different _implementations_ leading to identical _metadata_.

If, like in the example above, the _metadata_ are different, we will need the help of another method to know more about the differences.

### `reduced_differ`

Here we are diffing **the metadata**.

The reduced_differ looks for specific aspects between the metadata of the 2 runtimes.
