TARGET = stm32f401_embassy
SOURCE = src/main.rs
.PHONY: bear clean
UBUNTU_VERSION := $(shell lsb_release -rs)
CARGO = Cargo.toml
CONF = .cargo/config.toml
DEBUG = 1
# Optimization level, can be [0, 1, 2, 3, s].
OPT = -O0
# Build path
MODE = release
BUILD_DIR = target/thumbv7em-none-eabi/$(MODE)
DEVICE = STM32F401RE
#######################################
# binaries
#######################################
PREFIX = arm-none-eabi-
# The gcc compiler bin path can be either defined in make command via GCC_PATH variable (> make GCC_PATH=xxx)
# either it can be added to the PATH environment variable.
CC = arm-none-eabi-gcc
AS = arm-none-eabi-gcc -x assembler-with-cpp
CP = arm-none-eabi-objcopy
SZ = arm-none-eabi-size
HEX = $(CP) -O ihex
BIN = $(CP) -O binary -S

#######################################
# ALL
#######################################

# default action: build all
all: $(BUILD_DIR)/$(TARGET).hex $(BUILD_DIR)/$(TARGET).bin

#######################################
# build the application
#######################################
# 根据mode编译
$(BUILD_DIR)/$(TARGET):$(SOURCE)
ifeq ($(MODE),release)
	cargo build --bin $(TARGET) --release
else
	cargo build --bin $(TARGET)
endif

$(BUILD_DIR)/%.hex: $(BUILD_DIR)/$(TARGET)
	$(HEX) $< $@
	
$(BUILD_DIR)/%.bin: $(BUILD_DIR)/$(TARGET)
	$(BIN) $< $@	
	
#######################################
# clean up
#######################################
clean:
	-rm -fR $(BUILD_DIR)

#######################################
# dependencies
#######################################
-include $(wildcard $(BUILD_DIR)/*.d)

debug:
# 需要下载tmux
	tmux has-session -t test0 2>/dev/null && tmux kill-session -t test0 || true
	tmux new-session -d -s test0 'openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c "flash write_image erase build/learn_startup.bin 0x8000000" -c reset'
# 这里看你电脑的性能,因为gdb-server服务器要启动一会儿
	sleep 5
	gdb-multiarch -x .gdbinit
format:
	find . -iname *.h -o -iname *.c | xargs clang-format -i
STdownload:
	openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c init -c "halt" -c "flash write_image erase $(BUILD_DIR)/$(TARGET).bin 0x8000000" -c "reset" -c "shutdown"
Jdownload:
# openocd -f interface/jlink.cfg -c "transport select swd" -f target/stm32f4x.cfg -c init -c "reset" -c "halt" -c "flash write_image erase $(BUILD_DIR)/$(TARGET).bin 0x8000000" -c "reset" -c "shutdown"
	JLinkExe -device $(DEVICE) -autoconnect 1 -if SWD -speed 4000 -CommanderScript script/JLinkDownload.jlink
# JLinkGDBServer -device $(DEVICE) -if swd -speed 4000 &
pico:
	picocom --omap crcrlf --imap lfcrlf -c -b 115200 /dev/ttyACM0
client:
# cat /dev/ttyACM0 | defmt-print -e target/thumbv7em-none-eabi/release/stm32f401_embassy 
	nc localhost 19021 | defmt-print -e $(BUILD_DIR)/$(TARGET)
server:
	