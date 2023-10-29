## Load testing to playground
1. `docker compose up -d --build grafana influxdb app`
2. `docker compose run k6 run /scripts/script.js`
3. You can see grafana dashboard on http://localhost:3000

