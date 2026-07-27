import re
from typing import Dict, List


_loaded_tables: Dict[str, List[int]] = {}


def load(file_path: str) -> Dict[str, List[int]]:
    """
    Load all C arrays from a file and keep them in memory.
    Returns a dictionary: {table_name: [values]}.
    """
    global _loaded_tables

    with open(file_path, "r", encoding="utf-8") as file:
        content = file.read()

    # Matches: const unsigned short TableName[] = { ... };
    pattern = re.compile(
        r"const\s+unsigned\s+short\s+([A-Za-z0-9_\-]+)\s*\[\]\s*=\s*\{(.*?)\};",
        re.DOTALL,
    )

    tables: Dict[str, List[int]] = {}
    for name, raw_values in pattern.findall(content):
        values = _parse_values(raw_values)
        tables[name] = values

    _loaded_tables = tables
    return _loaded_tables


def get_table_names() -> List[str]:
    """Return the names of loaded tables."""
    return list(_loaded_tables.keys())


def load_table(table_name: str) -> List[int]:
    """Return one table by name."""
    if table_name not in _loaded_tables:
        raise KeyError(f"Unknown table: {table_name}")
    return _loaded_tables[table_name]


def get_value(table: List[int], index: int) -> int:
    """Return one value from a table by index."""
    return table[index]


def _parse_values(raw_values: str) -> List[int]:
    # Remove both // and /* ... */ comments so numbers are parsed cleanly.
    without_block_comments = re.sub(r"/\*.*?\*/", "", raw_values, flags=re.DOTALL)
    without_comments = re.sub(r"//.*", "", without_block_comments)
    tokens = [token.strip() for token in without_comments.split(",")]

    values: List[int] = []
    for token in tokens:
        if not token:
            continue
        values.append(int(token, 0))
    return values
