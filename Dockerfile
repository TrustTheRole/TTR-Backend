FROM rust:latest

# Install dockerize
RUN apt-get update && apt-get install -y wget
RUN wget https://github.com/jwilder/dockerize/releases/download/v0.6.1/dockerize-linux-amd64-v0.6.1.tar.gz
RUN tar xzf dockerize-linux-amd64-v0.6.1.tar.gz -C /usr/local/bin

WORKDIR /usr/src/app

COPY . .
RUN cargo install diesel_cli --no-default-features --features postgres

ENV PATH="/root/.cargo/bin:${PATH}"
RUN chmod +x /usr/src/app/entrypoint.sh
EXPOSE 8080
ENTRYPOINT ["/usr/src/app/entrypoint.sh"]
