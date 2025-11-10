import mokaccino_py as mp
from mokaccino_py import Query

def test_simple():
    assert mp.sum_as_string(3, 5) == "8"

    assert Query.from_kv("field", "value") is not None