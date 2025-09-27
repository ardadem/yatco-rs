# Makefile for yatco-rs

SHELL := /bin/sh
PKG_MANAGER := npm
CARGO := cargo

.PHONY: help deps dev build

help:
	@printf "Available targets:\n"
	@printf "  deps            Install only node deps\n"
	@printf "  dev             Run the application in development mode\n"
	@printf "  build           Build the application\n"

deps:
	@echo "Installing JS deps with $(PKG_MANAGER)..."
	@if [ "$(PKG_MANAGER)" = "pnpm" ]; then pnpm install; \
	elif [ "$(PKG_MANAGER)" = "yarn" ]; then yarn install; \
	else npm ci || npm install; fi

dev:
	@echo "Running Tauri dev"
	$(PKG_MANAGER) run tauri dev

build:
	@echo "Running Tauri build"
	@$(PKG_MANAGER) run tauri build
