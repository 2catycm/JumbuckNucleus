## 0.0 启用bash功能，比如source，方便脚本编写
SHELL :=/bin/bash
## 0.1 是否对make的流程打日志
MAKE_SAY ?= true  # true
echo := echo
ifneq ($(MAKE_SAY), true)
	$(echo):= ":"
endif

# 1.1 构建目标
TARGET := riscv64gc-unknown-none-elf
MODE ?= dev
CARGO_BUILD_MODE := --profile=$(MODE)
ifeq ($(MODE), bench)
	RUSTC_MODE := release
else
	ifeq ($(MODE), release)
		RUSTC_MODE := release
	endif
	RUSTC_MODE := debug
endif
OS_NAME := jumbuck_nucleus
## 1.2 推导出编译的二进制位置，分ELF格式(用于debug)和BIN格式(用于加载到qemu)
# cargo编译生成
KERNEL_ELF := target/$(TARGET)/$(RUSTC_MODE)/$(OS_NAME)
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := target/$(TARGET)/$(RUSTC_MODE)/asm


# 1.3 操作系统的基础执行环境：监督层二进制接口(Supervisor Binary Interface)的定义
# 1.3.1 选用的SBI的名称，应当放置同名bin扩展名文件在bootloader文件夹下。可选参数，默认为qemu下模拟运行 rustsbi 。
PAGE_MODE ?= is_16KiB
BOARD ?= qemu
# default
SBI ?=rustsbi-$(BOARD)
BOOTLOADER := ../bootloader/$(SBI).bin
ifeq ($(SBI), default)
	BOOTLOADER := default
endif
# 1.3.2 SBI 内存布局
# 开发板大小
K210_BOOTLOADER_SIZE := 131072
# 内核入口点
ifeq ($(BOARD), qemu)
	KERNEL_ENTRY_PA := 0x80200000
else ifeq ($(BOARD), k210)
	KERNEL_ENTRY_PA := 0x80020000
endif

# Run K210
K210-SERIALPORT	= /dev/ttyUSB0
K210-BURNER = ../tools/kflash.py

# 2. 工具
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Disassembly
DISASM ?= -x

# Run usertests or usershell
TEST ?=

#build: env switch-check $(KERNEL_BIN)
build: $(KERNEL_BIN)

switch-check:
ifeq ($(BOARD), qemu)
	(which last-qemu) || (rm -f last-k210 && touch last-qemu && make clean)
else ifeq ($(BOARD), k210)
	(which last-k210) || (rm -f last-qemu && touch last-k210 && make clean)
endif

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils --vers =0.3.3
	rustup component add rust-src
	rustup component add llvm-tools-preview

$(KERNEL_BIN): kernel
	@$(echo) "内核构建成功。正在从elf格式导出bin格式。"
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@
	@$(echo) "成功构建 $(KERNEL_BIN)"

kernel:
	@#cd ../user && make build TEST=$(TEST)
	@make -C ../user build TEST=$(TEST)
	@$(echo) 平台为: $(BOARD)
	@cp src/linker-$(BOARD)-$(PAGE_MODE).ld src/linker.ld
	@cargo build $(CARGO_BUILD_MODE) --features "board_$(BOARD), $(PAGE_MODE)"
	@rm src/linker.ld

clean:
	@cargo clean

disasm: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less

disasm-vim: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@vim $(DISASM_TMP)
	@rm $(DISASM_TMP)

run: build
	@$(echo) "正在启动qemu模拟器。"
ifeq ($(BOARD),qemu)
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file="$(KERNEL_BIN)",addr=$(KERNEL_ENTRY_PA)
else
	(which $(K210-BURNER)) || (cd .. && git clone https://github.com/sipeed/kflash.py.git && mv kflash.py tools)
	@cp $(BOOTLOADER) $(BOOTLOADER).copy
	@dd if=$(KERNEL_BIN) of=$(BOOTLOADER).copy bs=$(K210_BOOTLOADER_SIZE) seek=1
	@mv $(BOOTLOADER).copy $(KERNEL_BIN)
	@sudo chmod 777 $(K210-SERIALPORT)
	python3 $(K210-BURNER) -p $(K210-SERIALPORT) -b 1500000 $(KERNEL_BIN)
	python3 -m serial.tools.miniterm --eol LF --dtr 0 --rts 0 --filter direct $(K210-SERIALPORT) 115200
endif

debug: build
	@tmux new-session -d \
		"qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

debug_server: build
	@qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S

debug_client:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

.PHONY: build env kernel clean disasm disasm-vim run switch-check gdbserver gdbclient

# cargo
fmt:
	@cargo fmt
check:
	@cargo check
doc:
	@cargo doc --document-private-items --open
# 测试 软件工程 语法
test_makefile_can_sub_var ?= 10
test_makefile_can_sub:
	echo "Greeting from sub makefile! "
	echo "var in sub makefile is $(test_makefile_can_sub_var)"
