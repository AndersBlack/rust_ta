FROM ubuntu:latest
COPY . /technical_application
WORKDIR /technical_application
RUN apt-get update -y && \
  apt-get upgrade -y && \
  apt-get install -y && \
  apt-get install -y cargo && \
  cargo build