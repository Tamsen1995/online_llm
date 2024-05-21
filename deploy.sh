#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

# Ensure necessary environment variables are set
if [ -z "$HEROKU_API_KEY" ]; then
  echo "HEROKU_API_KEY is not set. Please set it and try again."
  exit 1
fi

if [ -z "$HEROKU_APP_NAME" ]; then
  echo "HEROKU_APP_NAME is not set. Please set it and try again."
  exit 1
fi

# Log in to Heroku Container Registry
echo "Logging in to Heroku Container Registry..."
echo "$HEROKU_API_KEY" | docker login --username=_ --password-stdin registry.heroku.com

# Build the Docker image
echo "Building the Docker image..."
docker build -t registry.heroku.com/$HEROKU_APP_NAME/web .

# Push the Docker image to Heroku Container Registry
echo "Pushing the Docker image to Heroku Container Registry..."
docker push registry.heroku.com/$HEROKU_APP_NAME/web

# Release the Docker image
echo "Releasing the Docker image..."
curl -n -X PATCH https://api.heroku.com/apps/$HEROKU_APP_NAME/formation \
  -d '{
    "updates": [
      {
        "type": "web",
        "docker_image": "'$(docker inspect registry.heroku.com/$HEROKU_APP_NAME/web:latest --format={{.Id}})'"
      }
    ]
  }' \
  -H "Content-Type: application/json" \
  -H "Accept: application/vnd.heroku+json; version=3.docker-releases" \
  -H "Authorization: Bearer $HEROKU_API_KEY"

echo "Deployment to Heroku completed successfully!"
