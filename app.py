from flask import Flask, jsonify, request, render_template
from models import db, Task

app = Flask(__name__)
app.config.from_object('config')
db.init_app(app)

with app.app_context():
    db.create_all()

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/api/tasks', methods=['GET'])
def list_tasks():
    tasks = Task.query.order_by(Task.priority.desc(), Task.created_at.desc()).all()
    return jsonify([t.to_dict() for t in tasks])

@app.route('/api/tasks', methods=['POST'])
def create_task():
    data = request.get_json()
    if not data or not data.get('title'):
        return jsonify({'error': 'Title is required'}), 400
    task = Task(
        title=data['title'].strip(),
        description=data.get('description', ''),
        category=data.get('category', 'general'),
        priority=data.get('priority', 0),
    )
    db.session.add(task)
    db.session.commit()
    return jsonify(task.to_dict()), 201

@app.route('/api/tasks/<int:task_id>', methods=['GET'])
def get_task(task_id):
    task = Task.query.get(task_id)
    if not task:
        return jsonify({'error': 'Task not found'}), 404
    return jsonify(task.to_dict())

@app.route('/api/tasks/<int:task_id>', methods=['PUT'])
def update_task(task_id):
    task = Task.query.get(task_id)
    if not task:
        return jsonify({'error': 'Task not found'}), 404
    data = request.get_json()
    if data.get('title') is not None:
        task.title = data['title'].strip()
    if data.get('description') is not None:
        task.description = data['description']
    if data.get('category') is not None:
        task.category = data['category']
    if data.get('priority') is not None:
        task.priority = data['priority']
    db.session.commit()
    return jsonify(task.to_dict())

@app.route('/api/tasks/<int:task_id>', methods=['DELETE'])
def delete_task(task_id):
    task = Task.query.get(task_id)
    if not task:
        return jsonify({'error': 'Task not found'}), 404
    db.session.delete(task)
    db.session.commit()
    return jsonify({'message': 'Task deleted'})

@app.route('/api/tasks/<int:task_id>/toggle', methods=['PATCH'])
def toggle_task(task_id):
    task = Task.query.get(task_id)
    if not task:
        return jsonify({'error': 'Task not found'}), 404
    task.status = 'done' if task.status == 'pending' else 'pending'
    db.session.commit()
    return jsonify(task.to_dict())

if __name__ == '__main__':
    app.run(debug=True, port=5000)
