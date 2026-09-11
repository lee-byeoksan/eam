def label(text: str) -> str:
    normalized = " ".join(text.split())
    if not normalized:
        raise ValueError("label must not be empty")
    return normalized
