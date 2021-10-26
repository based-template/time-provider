# time-provider Makefile

CAPABILITY_ID = "auxiliary::interfaces::time"
NAME = "time-provider"
VENDOR = "OMT"
PROJECT = time_provider
VERSION = 0.2.0
REVISION = 0

include ./provider.mk

test:
	cargo test -- --nocapture

