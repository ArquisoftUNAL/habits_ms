docker-build-db-master:
	docker build -t habitus_habits_db_master .

docker-run-db-master:
	docker run -d --name habitus_habits_db_master -p 5432:5432 --env-file .env habitus_habits_db_master