docker-build-db:
	docker build -t habitus_habits_db .

docker-run-db:
	docker run -d -t -i -p 5432:5432 --name habitus_habits_db --env-file .env habitus_habits_db