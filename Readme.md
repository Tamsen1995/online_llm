# Online LLM Project

## Overview

This project is a RESTful, OpenAI-compatible API built using Rust. It's designed to integrate with any Large Language Model (LLM) from openAI you input into it, providing flexibility. The application utilizes Docker and Docker Compose for easy deployment and scalability. It also features SerpApi for web scraping to enhance the context provided to the language model. The design prioritizes scalability, security, and ease of deployment.

## Features

- **Web Scraping**: Integrates with SerpApi to fetch relevant web data.
- **Dockerized**: Fully containerized using Docker and Docker Compose for easy setup and deployment.
- **Health Checks**: Implements health checks to ensure the service is running correctly.
- **Environment Configuration**: Utilizes environment variables for configuration management.

## Project Structure

```plaintext
online_llm/
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── Rocket.toml
├── docker-compose.yml
├── .env
├── src/
│   ├── api.rs
│   ├── config.rs
│   ├── main.rs
│   ├── models.rs
│   ├── openai_client.rs
│   └── serpapi_client.rs
└── README.md
```

### File Descriptions

- **Cargo.toml**: Rust project configuration file.
- **Dockerfile**: Instructions for building the Docker image.
- **Rocket.toml**: Configuration file for the Rocket web framework.
- **docker-compose.yml**: Docker Compose file to set up the services.
- **.env**: Environment variables file.
- **src/**: Source directory containing the Rust code.
  - **api.rs**: Defines the API endpoints.
  - **config.rs**: Handles configuration and environment variables.
  - **main.rs**: Entry point of the application.
  - **models.rs**: Data models for the application.
  - **openai_client.rs**: Integrates with OpenAI's API.
  - **serpapi_client.rs**: Integrates with SerpApi for web scraping.

## Setup and Running Locally

### Prerequisites

- Docker
- Docker Compose

### Steps

1. **Create the `.env` file**:

   ```sh
   touch .env
   ```

   Add the following lines to the `.env` file with your API keys:

   ```env
   OPENAI_API_KEY=your_openai_api_key_here
   SERPAPI_API_KEY=your_serpapi_api_key_here
   ```

2. **Build and run the application**:

   ```sh
   docker-compose build
   docker-compose up
   ```

   When running the application with docker-compose up, the health check endpoint is hit every 30 seconds. You should see the OK log in the terminal in which you launched the app.

3. **Verify the application is running**:

   - Check the Docker container logs to ensure the Rocket server is running:
     ```sh
     docker-compose logs
     ```
   - You should see `Rocket has launched from http://0.0.0.0:8000`.

## Running without Docker Compose

If you want to run the application without Docker Compose, you can use the run.sh script that is included in the repository. To do this, simply run the following command:

```sh
./run.sh
```

## Health Check

The application includes a health check endpoint to monitor its status. The health check endpoint is `/health` and can be used to verify that the application is running correctly.

### Example

```sh
curl http://localhost:8000/health
```

This should return `OK` if the application is running correctly.

## Deployment to Heroku

This repository includes a script for deploying the application to Heroku using the Heroku Container Registry. The script is named `deploy.sh`.

### Prerequisites

- Docker
- Heroku CLI
- Heroku account

### Environment Variables

Before running the script, you need to set the following environment variables:

- `HEROKU_API_KEY`: Your Heroku API key. You can get this from your Heroku account settings.
- `HEROKU_APP_NAME`: The name of your Heroku app. This is the name you chose when you created the app on Heroku.

You can set these environment variables in your shell like this:

```sh
export HEROKU_API_KEY=your_heroku_api_key_here
export HEROKU_APP_NAME=your_heroku_app_name_here
```

Running the Script

```sh
./deploy.sh
```

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.
