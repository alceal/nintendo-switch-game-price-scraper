FROM rust:1.74.0-slim as base

# FROM base as build

# FROM base as final

WORKDIR /app/

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#user
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser

USER appuser

COPY . .

EXPOSE 8000

ENTRYPOINT ["bash"]