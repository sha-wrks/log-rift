def test_priority_sort():
    tasks = [{"priority": 2}, {"priority": 0}, {"priority": 1}]
    sorted_tasks = sorted(tasks, key=lambda t: -t["priority"])
    assert sorted_tasks[0]["priority"] == 2
    assert sorted_tasks[2]["priority"] == 0
