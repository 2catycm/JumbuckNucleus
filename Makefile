# 绵羊核心构建脚本 Makefile.
# @author 叶璨铭。
# Makefile 语法参考
# https://unix.stackexchange.com/questions/217295/phony-all-rules-in-gnu-make-file

## 0.0 启用bash功能，比如source，方便脚本编写
SHELL :=/bin/bash
## 0.1 是否对make的流程打日志
MAKE_SAY ?= false  # true
echo := echo
ifneq ($(MAKE_SAY), true)
	$(echo):= ":"
endif
.PHONY: default_makefile_target
default_makefile_target: run
# 1. 生命周期 test/run/clean/debug_server/check/qemu/fmt
.PHONY: test run r clean debug_server ds check qemu fmt doc
run:
	@$(echo) "运行项目。"
	@make -C os run
r: run
clean:
	@$(echo) "清理项目编译结果。"
	@$(echo) "正在清理子项目 os 。"
	@make -C os clean
	@$(echo) "正在清理子项目 user 。"
	@make -C user clean
debug_server:
	@$(echo) "启动项目 debug 服务器, 以便远程调试。"
	make -C os debug_server
ds: debug_server
check:
	@$(echo) "检查项目代码语法。"
	@make -C os check
	@make -C user check

qemu:
	@# TODO
	@$(echo) "尚未实现！ "

fmt:
	@$(echo) "设置项目代码格式。"
	@make -C os fmt
	@make -C user fmt
doc:
	@$(echo) "生成项目文档。"
	@make -C os doc
	@make -C user doc
# 2. 选择qemu版本
QEMU_VERSION ?=7.0-4Ki
QEMU_BUILD := $(shell pwd)/qemu-bin/qemu-$(QEMU_VERSION)
.PHONY: set_qemu
set_qemu: $(QEMU_BUILD)
    #TODO 有bug
	@$(echo) "正在设置Qemu为指定版本($(QEMU_VERSION))。"
	@source ./qemu-bin/set_qemu.bash
	qemu-as-$(QEMU_VERSION)
	@$(echo) "设置完成。"

# 3. docker 镜像导出
.PHONY: run_docker build_docker
DOCKER_NAME ?= dinghao188/rcore-tutorial

run_docker:
	docker run --rm -it --mount type=bind,source=$(shell pwd),destination=/mnt ${DOCKER_NAME}

build_docker:
	docker build -t ${DOCKER_NAME} .

# 4. 测试 Makefile 语法特性

test_makefile_can_sub:
	@$(echo) "测试 Makefile是否可以 对子项目正确运行任务。"
	@make -C os test_makefile_can_sub