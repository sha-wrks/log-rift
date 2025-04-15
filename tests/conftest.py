import pytest
@pytest.fixture
def sample_task():
    return {
        "id": 1,
        "title": "Sample task",
        "status": "pending",
        "priority": 0
    }
@pytest.fixture
def task_list(sample_task):
    return [sample_task]
