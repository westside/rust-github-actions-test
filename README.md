# Rust Test Repository

This repository contains a Rust project that demonstrates the use of GitHub Actions for continuous integration and deployment across multiple platforms (macOS, Windows, and Linux).

## Project Overview
This is a basic "Hello World" project written in Rust. Its main purpose is to showcase how to set up GitHub Actions for a Rust project, enabling automatic building, testing, and releasing across different operating systems.

## CI/CD with GitHub Actions

This project uses GitHub Actions to automatically build, test, and release the software for macOS, Windows, and Linux. The workflows are triggered on:

- Pushes to the `main` branch
- Pull requests to the `main` branch
- Pushing a tag starting with 'v' (e.g., v1.0.0)

### Workflow Details

There are three separate workflows, one for each supported platform:

1. `macos-build.yml`: Builds and tests the project on macOS
2. `windows-build.yml`: Builds and tests the project on Windows
3. `linux-build.yml`: Builds and tests the project on Ubuntu (Linux)

Each workflow performs the following steps:

1. Checks out the code
2. Sets up the Rust toolchain
3. Builds the project
4. Runs the tests
5. Creates a release archive (when a new tag is pushed)
6. Creates a GitHub release with the built artifacts (when a new tag is pushed)
