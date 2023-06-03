# Perfectly balanced (load)

## Starting mockserver instances

Mockserver instances are set up as docker containers using docker compose.
There is utility makefile to make the usage more convenient. \
You can start and stop mockserver instances as such:
```shell
make start
make stop
```
By default, 10 instances are ran, but you can specify desired amount with `CLONES` variable.
```shell
make start CLONES=20
```
Ports in `docker-compose.yml` are configured to accommodate up to 100 instances. \
You can also chain makefile commands to restart instantly, for example:
```shell
make stop start
make stop start CLONES=7
```

