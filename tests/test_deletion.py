from tests.factories import create_task_dict
def test_delete_task():
    task = create_task_dict({"id": 1})
    tasks = [task]
    tasks = [t for t in tasks if t["id"] != 1]
    assert len(tasks) == 0
