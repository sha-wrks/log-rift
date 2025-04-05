import json
class TestClient:
    def __init__(self, app):
        self.app = app
        self.client = app.test_client()
    def get_tasks(self):
        return json.loads(self.client.get("/api/tasks").data)
    def create_task(self, title="Test"):
        return json.loads(self.client.post("/api/tasks", json={"title": title}).data)
