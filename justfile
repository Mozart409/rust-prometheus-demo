alias d := dev

default:
  just --list

clear:
	clear

dev: clear
	cargo watch -c -x run

up: clear
	docker compose up -d --remove-orphans

down: clear
	docker compose down

loadtest: clear
	hey -n 1000 -c 100 http://localhost:3000/