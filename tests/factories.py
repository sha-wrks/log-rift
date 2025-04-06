def create_task_dict(overrides=None):
    task = {
        "id": 1,
        "title": "Test task",
        "description": "",
        "status": "pending",
        "priority": 0,
        "category": "general",
        "due_date": "",
        "created_at": "2025-01-01T00:00:00",
        "updated_at": "2025-01-01T00:00:00"
    }
    if overrides: task.update(overrides)
    return task
