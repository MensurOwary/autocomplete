# Autocomplete

Provide the following CLI functionality:

```bash
autocomplete> insert word
autocomplete> insert world
autocomplete> insert worst
autocomplete> insert worm
autocomplete> insert wreck
autocomplete> complete wr
autocomplete> ["wreck"]
autocomplete> complete wor
autocomplete> ["word", "world", "worst", "worm"]
autocomplete> complete wor --top 2 # not supported yet
autocomplete> ["word", "worm"]
```

Note: maybe we can work with frequencies as well. Currently it's just word completion based on the provided string, no frequency is taken into account. Inserting an existing word will simply be ignored