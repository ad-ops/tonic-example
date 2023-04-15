ARG BASE
FROM ${BASE} as build-env

FROM gcr.io/distroless/cc-debian11:nonroot
WORKDIR /app
COPY --from=build-env /app/target/release/server ./
USER nobody:nobody
CMD ["/app/server"]