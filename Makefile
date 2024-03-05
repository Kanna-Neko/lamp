.PHONY: lamp_api
lamp:
	cd api && cargo run

.PHONY: lamp_docker_up
lamp_docker_up:
	docker compose -f install/docker/lamp/docker-compose.yml up --build -d

.PHONY: lamp_docker_down
lamp_docker_down:
	docker compose -f install/docker/lamp/docker-compose.yml down