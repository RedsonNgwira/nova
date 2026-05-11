# Nova Build Environment Dockerfile
FROM ubuntu:22.04

# Avoid interactive prompts during build
ENV DEBIAN_FRONTEND=noninteractive

# Install core build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    git \
    autoconf \
    automake \
    libtool \
    pkg-config \
    nasm \
    ccache \
    python3 \
    python3-dev \
    openjdk-17-jdk \
    zip \
    unzip \
    libx11-dev \
    libxt-dev \
    libfontconfig1-dev \
    libglib2.0-dev \
    libxml2-dev \
    libxslt1-dev \
    libcups2-dev \
    libcurl4-openssl-dev \
    libgraphite2-dev \
    libicu-dev \
    liblcms2-dev \
    libnss3-dev \
    libpng-dev \
    libpoppler-cpp-dev \
    libboost-dev \
    libboost-iostreams-dev \
    libboost-system-dev \
    libboost-thread-dev \
    libboost-date-time-dev \
    libboost-test-dev \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Rust for the Lumen module
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set up ccache
ENV CCACHE_DIR=/root/.ccache
RUN ccache --max-size=10G

WORKDIR /home/nova/core

# Default command
CMD ["/bin/bash"]
