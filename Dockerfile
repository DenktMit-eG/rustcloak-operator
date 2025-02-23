FROM clux/muslrust:1.82.0-stable-2024-10-24 AS build

WORKDIR /src

COPY . .

RUN \
	mkdir -p /cargo/cargo && \
	ln -sf $HOME/.cargo/config /cargo/cargo && \
	CARGO_HOME=/cargo/cargo \
	CARGO_TARGET_DIR=/cargo/target \
	cargo install \
		--path rustcloak-operator \
		--root /app \
		--bin rustcloak

FROM gcr.io/distroless/static:nonroot

COPY --from=build /app/bin/rustcloak /

EXPOSE 8080

ENTRYPOINT ["/rustcloak"]
