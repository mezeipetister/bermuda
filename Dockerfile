# ====================
# Stage 1
# Build the API
# ====================
FROM rustlang/rust:nightly AS api
WORKDIR /app
COPY . /app/
RUN cargo build --bin api --release

# ====================
# Stage 2
# Build client
# ====================
FROM node:alpine3.11 as client
WORKDIR /app
ENV PATH /app/node_modules/.bin:$PATH
COPY client/package.json /app/package.json
RUN npm install
RUN npm install -g @angular/cli
COPY ./client/*.* /app

RUN ng build --outputPath=dist --prod --aot

# ====================
# Stage Final
# Bundle API and Client into a single container
# ====================
FROM ubuntu:latest AS api_server
WORKDIR /app
COPY --from=api /app/target/release/api .
# update for future dep install
RUN apt update
# Install libssl as dependency
RUN apt install libssl-dev -y
RUN apt install nginx -y
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=client /app/dist /usr/share/nginx/html
RUN systemctl reload nginx
ENTRYPOINT ["./api"]
EXPOSE 80/tcp