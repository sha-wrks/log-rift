def test_task_update():
    task = {"title": "Old", "status": "pending", "priority": 0}
    task["title"] = "Updated"
    task["priority"] = 1
    assert task["title"] == "Updated"
    assert task["priority"] == 1
