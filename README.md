# Tonic Example
This is a trivial example on how to setup a Rust workspace with a `gRPC` client/server in `kubernetes` using `skaffold`. The idea is that it should be simple to add more to this setup and automate most steps.

## Run
You can run both services using two terminals with `cargo run --bin server` and `cargo run --bin client`.

If you have `docker`, `kubernetes(k3s/minikube)` and `skaffold` installed you can run: `skaffold dev`. It should build, deploy and watch the whole project.

## Considerations
Docker-images take a long time to build with Rust. Smart caching layers elevates it a bit, but it still needs to rebuild too much.

An alternative might be to build locally and then just move the artifacts to docker similar how it would be with python, but there is a point in using the same as you would deploy it.

One issue with this approach is also that every change triggers a rebuild of the base-image, but on the other hand it speeds up easy.