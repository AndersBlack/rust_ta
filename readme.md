# Technical Assignment - Autorola
This repo contains a simple Rust CRUD REST API as part of a technical assignment for Autorola Denmark. The service was developed and tested on a linux, ubuntu distro. 

## Running the service
The service requires Rust, Cargo and some libraries (should be installed automatically). A dockerfile is available in the repo for convenience. 

To run the service when Rust and Cargo is installed use the command: 
```
cargo run
```

In the src/calls.rs the annotation can be seen for the different calls. They should be self-explanatory :smiley:

## Docker
A docker image can be build using the command

```
docker build . -t technical_application:latest
```

If the service is ran in a docker container, keep in mind to use the "--network host" when starting the container. This is so that the localhost:9090 is on the host machine network stack. An example of running the image interactable:

```
docker run --network host -it technical_application:latest
```

Once in the image, use:

```
cargo run 
```

This should expose the service on your host machine on localhost:9090.