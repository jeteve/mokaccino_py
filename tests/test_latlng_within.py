
import pytest
from mokaccino import Query, Percolator, Document

def test_latlng_within_query():
    # San Francisco coordinates approx
    lat = 37.7749
    lng = -122.4194
    radius_meters = 1000 # 1km

    # Create a query for location within 1km of SF center
    q = Query.from_latlng_within("location", lat, lng, radius_meters)

    p = Percolator()
    qid = p.add_query(q)

    # 1. Document at the center (should match)
    # The "location" field can be of the form <lat>,<long>
    doc_match = Document().with_value("location", f"{lat},{lng}")
    matches = p.percolate_list(doc_match)
    assert qid in matches

    # 2. Document far away
    # New York coordinates: 40.7128, -74.0060
    doc_no_match = Document().with_value("location", "40.7128,-74.0060")
    matches_ny = p.percolate_list(doc_no_match)
    assert qid not in matches_ny


def test_invalid_latlng():
    # Try a very invalid value
    with pytest.raises(RuntimeError, match="Invalid lat/lng"):
        Query.from_latlng_within("location", float('nan'), 0.0, 1000)

def test_negative_distance():
    # Negative distance should raise ValueError
    with pytest.raises(ValueError, match="Distance must be non-negative"):
        Query.from_latlng_within("location", 0.0, 0.0, -100)
