docker-build-ms:
	docker build -t habitus_habits_ms .

docker-run-ms:
	docker run -d --name habitus_habits_ms -p 3030:3030 --env-file .env habitus_habits_ms
