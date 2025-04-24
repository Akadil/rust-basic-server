#!/bin/bash

# Build and start the Docker containers
docker-compose up -d

# Display the logs
echo "Starting Docker containers..."
echo "To view logs, run: docker-compose logs -f"
echo "To stop the containers, run: docker-compose down"
echo ""
echo "The server with PostgreSQL repository is available at: http://localhost:8080"
echo "The server with in-memory repository is available at: http://localhost:8081"
