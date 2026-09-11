def label(text: str) -> str:
    if not isinstance(text, str):
        raise TypeError("label expects a str, got " + type(text).__name__)
    normalized = " ".join(text.split())
    if not normalized:
        raise ValueError("label must not be empty")
    return normalized
