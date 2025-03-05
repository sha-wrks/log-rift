.PHONY: build run stop logs shell clean test lint format

build: ; docker compose build
run: ; docker compose up -d
stop: ; docker compose down
logs: ; docker compose logs -f
shell: ; docker compose exec api bash
clean: ; docker compose down -v
test: ; pytest tests/ --cov=api/ -v
lint: ; flake8 api/
format: ; black --check api/
