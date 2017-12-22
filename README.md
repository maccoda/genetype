# Genetype

Small project to generate rust types from JSON objects. Requirement derived from
development of webhook that communicates via JSON payload, thought it would be
much better if these types were generated from the typical payload that should
be accessible.

This project is far from polished! Has only just reached working state for the one
I was basing it on. Use with caution and let me know where it needs improvements!


## Usage

```bash
$ cargo run -- /path/to/json
```
