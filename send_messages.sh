#!/bin/bash

for ((i=1; i<=10; i++))
do
    # Construct the message with the iteration number
    message="{\"msg\":\"Hello World - Iteration $i\"}"

    curl -X POST http://localhost:8000/ -H "Content-Type: application/json" -d "$message"

    echo
done
