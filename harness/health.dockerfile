# Health check configuration for Docker/Kubernetes

healthcheck:
  test: ["CMD", "python", "harness/scripts/health_server.py", "--health"]
  interval: 30s
  timeout: 10s
  retries: 3
  start_period: 10s

# Environment variables
ENV HELIOS_VERSION=0.1.0
ENV HELIOS_PORT=8080
ENV HELIOS_HOST=0.0.0.0

# Expose health ports
EXPOSE 8080 8081

# Health server runs on port 8081 by default
CMD ["python", "harness/scripts/health_server.py", "--port", "8081"]
