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

## Contributions

Contributions are welcome! Please fork the repository and create a pull request with your changes.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

---

This README provides a comprehensive overview of the project, setup instructions, and deployment steps. You can modify it as needed to fit your project's specifics and include any additional details you think are necessary.
