from mokaccino import Percolator, Query, Document


def test_percolator_works():
    p = Percolator()
    assert p is not None
    q =  Query.from_kv("field", "value")
    qid = p.add_query(q)
    assert qid == 0

    d = Document().with_value("field", "value")
    assert p.percolate(d) == [qid]
