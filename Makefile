.PHONY: build run stop logs shell clean

build:
	docker compose build

run:
	docker compose up -d

stop:
	docker compose down

logs:
	docker compose logs -f

shell:
	docker compose exec api bash

clean:
	docker compose down -v
