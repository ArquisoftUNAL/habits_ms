docker-build-db-img:
	docker build -t habitus_habitats_db ./database/

docker-run-db:
	docker run -d -t -i -p 5432:5432 --name habitus_habitats_db --env-file .env habitus_habitats_db

docker-build-ms:
	docker build -t habitus_habitats_ms .

docker-run-ms:
	docker run -d --name habitus_habitats_ms -p 3030:3030 --env-file .env habitus_habitats_ms
