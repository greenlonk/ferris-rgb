FROM mcr.microsoft.com/devcontainers/base:bullseye

# Install direnv deterministically from Debian repos
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get install -y --no-install-recommends direnv \
    && apt-get clean && rm -rf /var/lib/apt/lists/*