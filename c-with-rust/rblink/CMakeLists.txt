if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --target thumbv6m-none-eabi)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release --target thumbv6m-none-eabi)
    set(TARGET_DIR "release")
endif ()

set(RBLINK_A "${CMAKE_CURRENT_BINARY_DIR}/thumbv6m-none-eabi/${TARGET_DIR}/librblink.a")

add_custom_target(rblink ALL
    COMMENT "Compiling rblink module"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
    COMMAND cp ${RBLINK_A} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(rblink PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

add_test(NAME rblink_test
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
