from mokaccino_py import Document

def test_build():
    d = Document()
    assert d is not None
    d = d.with_value("field", "v1")
    d = d.with_value("field", "v2")
    d = d.with_value("field", "v3")
    d = d.with_value("taste", "sweet")
    assert d is not None
    assert d.field_values() == [("field", "v1"), ("field", "v2"), ("field" , "v3"), ("taste", "sweet")]
