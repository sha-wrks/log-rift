def paginate(items, page=1, per_page=20):
    start = (page - 1) * per_page
    end = start + per_page
    return {
        "items": items[start:end],
        "total": len(items),
        "page": page,
        "per_page": per_page,
        "pages": max(1, -(-len(items) // per_page))
    }
