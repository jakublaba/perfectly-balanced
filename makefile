CLONES=5

start:
	docker compose up --scale mockserver=$(CLONES) -d
stop:
	docker compose down
.PHONY: start stop
