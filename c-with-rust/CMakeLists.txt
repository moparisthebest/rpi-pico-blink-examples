cmake_minimum_required(VERSION 3.12)

# Pull in SDK (must be before project)
include(pico_sdk_import.cmake)

project(pico_examples C CXX ASM)
set(CMAKE_C_STANDARD 17)
set(CMAKE_CXX_STANDARD 17)

# Initialize the SDK
pico_sdk_init()

add_executable(
		blink
        blink.c
        )

add_subdirectory(rblink)
add_dependencies(blink rblink)

# pull in common dependencies
target_link_libraries(
	blink 
	pico_stdlib
	${CMAKE_CURRENT_BINARY_DIR}/rblink/librblink.a
	)

# create map/bin/hex file etc.
pico_add_extra_outputs(blink)
