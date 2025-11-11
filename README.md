This uses uv/uvx to handle the python side of things

1. Prepare the venv

```sh
uv venv --python 3.13
```

2. Compile everything using maturin

```sh
uvx maturin develop
```

3. Run some examples or unit tests

```sh
uv sync --extra dev
pytest
uv run examples/...
```


In development, loop through 3 and 2:

```sh
uvx maturin develop && pytest
```
