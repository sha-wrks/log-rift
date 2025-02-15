def validate_title(title):
    if not title or not title.strip():
        return 'Title is required'
    if len(title) > 200:
        return 'Title must be 200 characters or fewer'
    return None
