CC = gcc
CFLAGS = -Wall -Wextra -fpic

SRC_DIR = ./src
BUILD_DIR = ./build
VPATH = $(SRC_DIR)

SRCS = $(shell find $(SRC_DIR) -name "*.c" -or -name "*.s")
OBJS = $(patsubst $(SRC_DIR)/%.c,$(BUILD_DIR)/%.o,$(SRCS))
LIB = $(BUILD_DIR)/libexample.a

.PHONY: all
all: cparts
	cargo build

.PHONY: run
run: all
	cargo run

.PHONY: cparts
cparts: $(LIB)

$(BUILD_DIR)/%.o: %.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -o $@ -c $<

$(LIB): $(OBJS)
	ar rcs $@ $(OBJS)

.PHONY: clean
clean:
	rm -rf $(BUILD_DIR) $(OBJS)
	cargo clean
