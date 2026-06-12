# Ordo

[![Website](https://img.shields.io/badge/Website-ordo.yansha.dev-000000?style=flat-square)](https://ordo.yansha.dev)
[![Flask](https://img.shields.io/badge/Flask-3.1-000000?logo=flask&logoColor=white&style=flat-square)](https://flask.palletsprojects.com/)
[![Python](https://img.shields.io/badge/Python-3.14-3776AB?logo=python&logoColor=white&style=flat-square)](https://python.org/)
[![Backbone.js](https://img.shields.io/badge/Backbone.js-1.6-0071B5?logo=javascript&logoColor=white&style=flat-square)](https://backbonejs.org/)
[![Tailwind](https://img.shields.io/badge/Tailwind_CSS-06B6D4?logo=tailwindcss&logoColor=white&style=flat-square)](https://tailwindcss.com/)
[![GSAP](https://img.shields.io/badge/GSAP-88CE02?logo=greensock&logoColor=white&style=flat-square)](https://gsap.com/)
[![License](https://img.shields.io/badge/License-MIT-green.svg?style=flat-square)](LICENSE)

[![Website](https://img.shields.io/badge/Live-ordo.yansha.dev-000000?style=flat-square)](https://ordo.yansha.dev)
[![CI](https://img.shields.io/github/actions/workflow/status/shadvls/ordo/.github/workflows/ci.yml?branch=main&style=flat-square)](https://github.com/shadvls/ordo/actions)

A task management web app built with **Flask** on the backend and **Backbone.js** on the frontend.
Features real-time CRUD operations, category & priority filtering, dark mode, smooth GSAP animations,
and a command palette -- deployed serverlessly on Vercel.

---

## Project Structure

| Directory | App | Stack | Deployed at |
|---|---|---|---|
| `api/` | REST API | Python (Flask) + JSON file storage | Vercel - [ordo.yansha.dev/api](https://ordo.yansha.dev/api/health) |
| `static/` | Frontend | Backbone.js + Handlebars + Tailwind CSS + GSAP | Vercel - [ordo.yansha.dev](https://ordo.yansha.dev) |

---

## Tech Stack

### Backend

- **Framework:** Flask 3.1 (Python 3.14)
- **Storage:** JSON file (`/tmp/ordo-data/tasks.json`)
- **CORS:** flask-cors 6.0
- **Server:** Gunicorn (production), Werkzeug dev server (local)
- **Extras:** Rate limiting, request ID tracing, structured logging, Prometheus-compatible metrics, ETag caching

### Frontend

- **Framework:** Backbone.js 1.6 + Underscore
- **Templating:** Handlebars (precompiled in `index.html`)
- **Styling:** Tailwind CSS 3 + custom CSS variables (dark/light theme)
- **Animation:** GSAP (stagger list items, progress bar, confetti)
- **Icons:** Font Awesome 6 (free)
- **PWA:** `manifest.json` for installable web app

---

## Features

- **CRUD tasks** -- Create, read, update, delete with undo toast notification
- **Filter by category** -- General / Work / Personal / Learning pill filters
- **Priority sorting** -- Higher priority tasks appear first
- **Due dates** -- Overdue highlighting with relative time display ("2h ago")
- **Search** -- Debounced real-time search (150ms) across task titles
- **Dark mode** -- System preference detection + manual toggle (localStorage)
- **Command palette** -- `Cmd+K` quick actions (add task, search, toggle theme)
- **Keyboard shortcuts** -- `n` quick add, `/` search, `Cmd+K` palette
- **Animations** -- GSAP staggered reveals, progress bar, confetti celebration
- **Responsive** -- Mobile-first with glassmorphism header design

---

## Prerequisites

- Python 3.14 (or 3.12+)
- pip

---

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/shadvls/ordo.git
cd ordo
```

### 2. Install dependencies

```bash
pip install -r requirements.txt
```

### 3. Run the development server

```bash
python api/index.py
```

The app starts on `http://localhost:5000`.

### 4. Run tests

```bash
pytest tests/
```

---

## Available Commands

| Command | Description |
|---|---|
| `python api/index.py` | Start Flask dev server (port 5000) |
| `pytest tests/` | Run test suite |
| `flake8 api/` | Lint API modules |
| `python -m pytest tests/ --cov=api/` | Run tests with coverage |

---

## API

### Base URL

`/api/`

### Endpoints

| Method | Path | Description |
|---|---|---|
| `GET` | `/api/tasks` | List all tasks (supports `?limit=` & `?offset=`) |
| `POST` | `/api/tasks` | Create a task |
| `GET` | `/api/tasks/<id>` | Get a single task |
| `PUT` | `/api/tasks/<id>` | Update a task |
| `DELETE` | `/api/tasks/<id>` | Delete a task |
| `PATCH` | `/api/tasks/<id>/toggle` | Toggle task status |
| `GET` | `/api/health` | Health check |
| `GET` | `/api/metrics` | Basic metrics (uptime, response time) |

### Task Schema

```json
{
  "id": 1,
  "title": "Buy groceries",
  "description": "Milk, eggs, bread",
  "status": "pending",
  "priority": 0,
  "category": "personal",
  "due_date": "2026-06-15",
  "created_at": "2026-06-12T12:00:00",
  "updated_at": "2026-06-12T12:00:00"
}
```

---

## Linting & Quality Tools

| Tool | Config | Purpose |
|---|---|---|
| **Flake8** | `.flake8` | Python linting |
| **Pytest** | `pytest.ini` | Testing framework |
| **Coverage** | `.coveragerc` | Test coverage reporting |
| **Prettier** | `.prettierrc` | Code formatting |
| **Bandit** | `.bandit` | Python security linter |
| **Pylint** | `.pylintrc` | Python code analysis |
| **Mypy** | `mypy.ini` | Type checking (optional) |
| **Gitleaks** | GitHub Actions | Secret scanning |

---

## Deployment

| App | Platform | URL |
|---|---|---|
| API + Frontend | Vercel | [ordo.yansha.dev](https://ordo.yansha.dev) |
| API Health | Vercel | [ordo.yansha.dev/api/health](https://ordo.yansha.dev/api/health) |

Deployed via Vercel for GitHub -- pushes to `main` trigger automatic redeployment.

---

## Architecture Decisions

- **Single-page app** -- Backbone.js router handles all client-side navigation; no page reloads.
- **JSON file storage** -- No database dependency; tasks persist to `/tmp/ordo-data/tasks.json` (ephemeral on Vercel, persistent in dev).
- **Serverless Flask** -- `api/index.py` exports a WSGI `app` for Vercel's Python runtime; all other `api/*.py` modules are bundled as dependencies.
- **Tailwind via CDN** -- No build step; Tailwind CSS loaded from CDN in `index.html` with utility classes only.
- **Undo pattern** -- Deleted tasks show an undo toast with 4-second auto-dismiss; no confirmation dialogs.
- **Command palette** -- Modal overlay with keyboard navigation (ArrowUp/Down/Enter) and action dispatch.

---

## Security

- No authentication (task manager is single-user by design).
- All user input is validated server-side (title required, max 200 chars).
- CORS configured for development origins.
- Rate limiting on API endpoints (100 req/min per IP).
- Gitleaks scans every push + weekly.
- Dependencies audited via Trivy (GitHub Actions security workflow).

---

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for the full contribution guide.

Please follow **Conventional Commits** (`feat:`, `fix:`, `chore:`, `refactor:`, `docs:`).

---

## License

**MIT License** -- Copyright (c) 2026 Yansha de Valois. See [LICENSE](./LICENSE) for details.
