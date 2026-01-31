FROM docker.io/rust:1.93.0

# Create the user to match your host (Assuming UID 1000)
RUN useradd -m -u 1000 james

USER james
ENV PATH="/home/james/.local/bin:${PATH}"


RUN curl -fsSL https://claude.ai/install.sh | bash
RUN curl -sSL https://raw.githubusercontent.com/steveyegge/beads/main/scripts/install.sh | bash

WORKDIR /workspace
