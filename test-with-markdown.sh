#!/bin/bash
# Run tests with markdown linting enabled
export $(cat .env)
cargo test "$@"
