import pytest as pytest

from main import has_double_characters


@pytest.mark.parametrize(
    'input, has_double', [
        ("fuck", False),
        ("hello", True),
    ]
)
def test_has_double_character(input: str, has_double: bool):
    assert has_double_characters(input) == has_double