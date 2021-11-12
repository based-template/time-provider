# time-provider Makefile

CAPABILITY_ID = "auxiliary::interfaces::time"
NAME = "time-provider"
VENDOR = "OMT"
PROJECT = time_provider
VERSION = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[].version' | head -1)
REVISION = 0

include ./provider.mk

test:
	cargo test -- --nocapture

