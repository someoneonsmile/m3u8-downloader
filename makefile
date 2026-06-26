.PHONY: build release run test check fmt lint clean install cross-build help

# 默认目标: 帮助
.DEFAULT_GOAL := help

# 编译参数
CARGO := cargo
RELEASE_NAME := m3u8-downloader

## build: 编译 debug 版本
build:
	$(CARGO) build

## release: 编译 release 版本 (优化+裁剪)
release:
	$(CARGO) build --release

## run: 编译并运行 (需传 url= 和 dest= 参数)
run:
	$(CARGO) run -- --url $(url) --dest $(dest)

## test: 运行测试
test:
	$(CARGO) test

## check: 快速检查编译 (不生成二进制, 比 build 快)
check:
	$(CARGO) check

## fmt: 格式化代码
fmt:
	$(CARGO) fmt --all

## fmt-check: 检查代码格式 (CI 用)
fmt-check:
	$(CARGO) fmt --all -- --check

## lint: 运行 clippy 检查
lint:
	$(CARGO) clippy --all-features -- -D warnings

## fix: 自动修复 clippy 建议
fix:
	$(CARGO) clippy --all-features --fix --allow-dirty --allow-staged

## clean: 清理编译产物
clean:
	$(CARGO) clean

## install: 安装到系统 (release 模式)
install:
	$(CARGO) install --path . --force

## cross-build: 交叉编译 musl 静态链接 (需安装 cross)
cross-build:
	cross build --release --target x86_64-unknown-linux-musl

## cross-build-macos: 交叉编译 macOS (需安装 cross)
cross-build-macos:
	cross build --release --target x86_64-apple-darwin

## update: 更新依赖
update:
	$(CARGO) update

## outdated: 检查过期依赖 (需 cargo-outdated)
outdated:
	cargo outdated

## audit: 安全审计依赖 (需 cargo-audit)
audit:
	cargo audit

## help: 显示帮助
help:
	@echo "m3u8-downloader 常用命令:"
	@echo ""
	@grep -hE '^## ' $(MAKEFILE_LIST) | sed 's/^## //' | column -t -s ':'
