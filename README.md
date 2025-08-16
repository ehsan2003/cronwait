# cronwait

[![Crates.io](https://img.shields.io/crates/v/cronwait.svg)](https://crates.io/crates/cronwait)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple command-line utility to wait for the next cron schedule. `cronwait` is designed to simplify running scheduled tasks in environments like Docker containers, where setting up a full cron daemon can be cumbersome.

## Why `cronwait`?

Running cron jobs inside a Docker container is not straightforward. It often requires installing a cron daemon, managing its process, and handling log output, which goes against the "one process per container" philosophy.

`cronwait` offers a lightweight alternative. It's a small, single binary that does one thing: it pauses execution until the next scheduled time defined by a cron expression. This allows you to create simple, cron-like behavior in your shell scripts without the overhead of a cron daemon.

## Installation

### From Crates.io

If you have the Rust toolchain installed, you can install `cronwait` directly from crates.io:
```bash
cargo install cronwait
```

### In Docker

For Docker images, you can use a multi-stage build to compile `cronwait` and copy the binary to a minimal final image. This keeps your image size small.

Here is an example `Dockerfile`:
```dockerfile
# ---- Builder Stage ----
FROM rust:1-slim as builder

# Install cronwait from crates.io
RUN cargo install cronwait

# ---- Final Stage ----
FROM debian:slim

# Copy the cronwait binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/cronwait /usr/local/bin/cronwait

# Copy your task script
COPY your-task.sh /usr/local/bin/your-task.sh
RUN chmod +x /usr/local/bin/your-task.sh

# Set up an entrypoint script to run the cron loop
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
```

Your `entrypoint.sh` would contain the cron loop (see below).

## Usage

The basic usage is to provide a cron expression as an argument. `cronwait` will then sleep until the next time that matches the expression.

```bash
cronwait "<cron_expression>"
```

The cron expression format is standard: `sec min hour day_of_month month day_of_week`.

### Options

- `-p, --print [PRECISION]`: Print the time in seconds until the next cron schedule instead of waiting. An optional precision for the fractional part can be provided.
- `-q, --quiet`: Suppress the "Waiting for..." message.

### Examples

Wait until the next minute:
```bash
cronwait "0 * * * * *"
```

Wait until 2 AM every day:
```bash
cronwait "0 0 2 * * *"
```

Show how many seconds until the next minute, with 3 decimal places of precision:
```bash
cronwait --print 3 "0 * * * * *"
```

## Emulating a Cron Job Runner

You can use `cronwait` in a shell script to create a simple, continuous job scheduler.

```bash
#!/bin/sh

while true; do
    # Wait until the next scheduled time (e.g., every 5 minutes)
    cronwait "0 */5 * * * *"
    
    # Execute your job
    /path/to/your/job.sh
done
```

### Handling Long-Running Jobs

If your job might take longer to run than the cron interval, it could cause you to miss a scheduled run. To avoid this, you can run the job in the background:

```bash
#!/bin/sh

while true; do
    cronwait "0 */5 * * * *"
    /path/to/your/job.sh & # Note the '&' to run in the background
done
```
This ensures `cronwait` starts waiting for the next interval immediately, regardless of how long the previous job takes.

### A Note on Reliability

`cronwait` is stateless. It does not persist any information between runs. If the script or container is stopped (e.g., due to a power outage or server reboot), it will not execute any missed jobs when it restarts. It will simply wait for the next scheduled time from the moment it's started again. This is suitable for tasks like periodic backups, where missing a single run is not critical, but may not be appropriate for high-reliability requirements.

## License

This project is licensed under the MIT License.
