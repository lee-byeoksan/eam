def label(text):
    if not isinstance(text, str):
        raise TypeError("label input must be a string")
    result = " ".join(text.split())
    if not result:
        raise ValueError("label must not be empty")
    return result
