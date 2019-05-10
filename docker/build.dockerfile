FROM archlinux/base:latest

RUN pacman -Syy
#RUN pacman -Sy yaourt
#RUN yaourt -Syy

# Install the packages
RUN pacman -S --noconfirm clang
RUN pacman -S --noconfirm musl
RUN pacman -S --noconfirm lz4
RUN pacman -S --noconfirm zeromq
RUN pacman -S --noconfirm grep
RUN pacman -S --noconfirm diffutils
RUN pacman -S --noconfirm gawk
RUN pacman -S --noconfirm make
RUN pacman -S --noconfirm file
RUN pacman -S --noconfirm automake
RUN pacman -S --noconfirm ninja
RUN pacman -S --noconfirm cmake

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup target add x86_64-unknown-linux-musl

RUN pacman -S --noconfirm openssl
RUN pacman -S --noconfirm pkgconf
#RUN yaourt -Sy package-name

# Install rust utils
RUN cargo install sccache
ENV RUSTC_WRAPPER sccache
RUN cargo install just

# The whole directory is supposed to be mounted as /work
# Cache should be mounted to /cache
ENV SCCACHE_DIR /cache
WORKDIR /work
CMD just build