# Compiler and Flags
CXX = g++
CXXFLAGS = -std=c++23 -O0 -Wall -Werror -g -DIS_SERVER=1
LDFLAGS = -shared -fPIC

# Root directory (relative to where we are in the build directory)
ROOT_DIR = ..

# Include Paths
INCLUDE_DIRS = -I$(ROOT_DIR)/src/runtime/StaticLibModule/public -I$(ROOT_DIR)/src/runtime/DynamicLibModule/public

# Source Files (using relative paths)
SERVER_SRC = $(ROOT_DIR)/src/server/ServerExecutable/main.cpp
STATIC_LIB_SRC = $(ROOT_DIR)/src/runtime/StaticLibModule/private/runtime_sl.cpp
DYNAMIC_LIB_SRC = $(ROOT_DIR)/src/runtime/DynamicLibModule/private/test.cpp

# Object Files
SERVER_OBJ = server/ServerExecutable/main.o
STATIC_LIB_OBJ = runtime/StaticLibModule/private/runtime_sl.o
DYNAMIC_LIB_OBJ = runtime/DynamicLibModule/private/test.o

# Output Files
RUNTIME_STATIC = RuntimeStaticLib.a
RUNTIME_DYNAMIC = RuntimeDynamicLib.so
SERVER_EXE = ServerExecutable

# Build Rules
all: $(SERVER_EXE)

# Create directories
server/ServerExecutable:
	mkdir -p $@

runtime/StaticLibModule/private:
	mkdir -p $@

runtime/DynamicLibModule/private:
	mkdir -p $@

# Compile object files
$(SERVER_OBJ): $(SERVER_SRC) | server/ServerExecutable
	$(CXX) $(CXXFLAGS) $(INCLUDE_DIRS) -c $< -o $@

$(STATIC_LIB_OBJ): $(STATIC_LIB_SRC) | runtime/StaticLibModule/private
	$(CXX) $(CXXFLAGS) $(INCLUDE_DIRS) -c $< -o $@

$(DYNAMIC_LIB_OBJ): $(DYNAMIC_LIB_SRC) | runtime/DynamicLibModule/private
	$(CXX) $(CXXFLAGS) $(INCLUDE_DIRS) -fPIC -c $< -o $@

# Link targets
$(SERVER_EXE): $(SERVER_OBJ) $(RUNTIME_STATIC) $(RUNTIME_DYNAMIC)
	$(CXX) $(CXXFLAGS) $(INCLUDE_DIRS) $^ -Wl,-rpath,'$$ORIGIN' -o $@

$(RUNTIME_STATIC): $(STATIC_LIB_OBJ)
	ar rcs $@ $^

$(RUNTIME_DYNAMIC): $(DYNAMIC_LIB_OBJ)
	$(CXX) $(CXXFLAGS) $(LDFLAGS) $^ -o $@

# Debug target
debug:
	@echo "SERVER_SRC: $(SERVER_SRC)"
	@echo "STATIC_LIB_SRC: $(STATIC_LIB_SRC)"
	@echo "DYNAMIC_LIB_SRC: $(DYNAMIC_LIB_SRC)"
	@echo "SERVER_OBJ: $(SERVER_OBJ)"
	@echo "STATIC_LIB_OBJ: $(STATIC_LIB_OBJ)"
	@echo "DYNAMIC_LIB_OBJ: $(DYNAMIC_LIB_OBJ)"
	@echo "Source files exist check:"
	@if [ -f $(SERVER_SRC) ]; then echo "  $(SERVER_SRC) exists"; else echo "  $(SERVER_SRC) MISSING"; fi
	@if [ -f $(STATIC_LIB_SRC) ]; then echo "  $(STATIC_LIB_SRC) exists"; else echo "  $(STATIC_LIB_SRC) MISSING"; fi
	@if [ -f $(DYNAMIC_LIB_SRC) ]; then echo "  $(DYNAMIC_LIB_SRC) exists"; else echo "  $(DYNAMIC_LIB_SRC) MISSING"; fi

# Clean
clean:
	rm -rf runtime server $(RUNTIME_STATIC) $(RUNTIME_DYNAMIC) $(SERVER_EXE)

.PHONY: all clean debug
