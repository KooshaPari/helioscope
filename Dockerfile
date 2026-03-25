FROM python:3.12-slim

WORKDIR /app

# Install uv
COPY --from=ghcr.io/astral-sh/uv:latest /uv /usr/local/bin/uv

# Copy project files
COPY pyproject.toml README.md ./
COPY app.py ./
COPY src ./src
COPY sample_data.py ./
COPY .env.example .env

# Install dependencies
RUN uv sync --frozen --no-dev

# Expose Streamlit port
EXPOSE 8501

# Run Streamlit
CMD ["python", "-m", "streamlit", "run", "app.py", "--server.port", "8501", "--server.address", "0.0.0.0"]
