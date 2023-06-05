#!/bin/bash

for i in {1..1000} ; do
	curl -s "localhost:8080/endpoint" > /dev/null
	# RESPONSE=$(curl -s "localhost:8080/endpoint")
	# echo -e "[$i]\t$RESPONSE"
done
