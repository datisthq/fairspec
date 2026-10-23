---
title: Python
label: Python
path: /python/library/
order: 20
icon: file-code
---

The Python packages provide a fast data management framework built on the Fairspec standard and Polars DataFrames. They support a wide range of tabular formats — CSV, TSV, JSON, JSONL, XLSX, ODS, Arrow, Parquet, SQLite — plus inline data and Jupyter Notebook integration.

The pages in this section show how to load, validate, and save tables in each format.

## Remote URLs

Loading functions fetch any `http(s)` URL they are given: descriptors, schemas and their `$ref`s, data files, and CKAN, Zenodo and GitHub datasets. A service handling untrusted URLs should refuse hosts that resolve to a loopback, private, link-local (cloud metadata at `169.254.169.254` among them) or other non-public address:

```python
from fairspec import set_host_policy

set_host_policy("public")  # None falls back to FAIRSPEC_HOST_POLICY
```

The check runs on the address actually connected to and again on every redirect, and proxy settings are ignored under it. Without a call, the `FAIRSPEC_HOST_POLICY` environment variable (`any` or `public`) decides, and the default is `any`. The MCP server starts with `public`.
