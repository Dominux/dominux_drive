run_dev:
	rm .env *.Dockerfile docker-compose.yml || true &&\
	cp ./deploy/dev/* . &&\
	cp ./deploy/dev/.env . &&\
	docker compose down --remove-orphans &&\
	docker compose up --build --force-recreate
