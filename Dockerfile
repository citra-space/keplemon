##############################
# Stage 1 – build the wheel  #
##############################
FROM ghcr.io/pyo3/maturin:latest AS builder

RUN yum install -y openssl-devel

# Build‑time arguments
ARG OS
ARG ARCH
ARG PYTHON_VERSION

ENV PATH="/opt/python/cp${PYTHON_VERSION}-cp${PYTHON_VERSION}/bin:$PATH"

RUN cargo install cargo-make
WORKDIR /io
COPY . .

RUN cargo make build-${OS}-${ARCH}

#################################
# Stage 2 – export the wheels   #
#################################
FROM scratch AS export
COPY --from=builder /io/target/wheels /wheels
