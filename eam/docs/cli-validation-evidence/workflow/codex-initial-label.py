def label(text):
    result = " ".join(text.split())
    if not result:
        raise ValueError("label must not be empty")
    return result
