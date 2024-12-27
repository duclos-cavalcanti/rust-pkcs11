cmake_minimum_required(VERSION 3.4 FATAL_ERROR)

project(client)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${PROJECT_SOURCE_DIR}/bin)
set(CMAKE_MODULE_PATH ${PROJECT_SOURCE_DIR}/cmake)
set (CMAKE_CXX_STANDARD 11)

find_package(PkgConfig REQUIRED)
find_package(Protobuf REQUIRED)

pkg_search_module(PROTOBUF REQUIRED protobuf)

if(NOT PROTOBUF_FOUND)
    message(FATAL_ERROR "PROTOBUF WAS NOT FOUND BY PKG-CONFIG.")
endif()

set(PROTO_BIN   protobin)
set(PROTO_FILES ${PROJECT_SOURCE_DIR}/../proto/message.proto)

PROTOBUF_GENERATE_CPP(
    PROTO_SRCS 
    PROTO_HDRS 
    ${PROTO_FILES}
)

message("PROTO_SRCS: ${PROTO_SRCS}")
message("PROTO_HDRS: ${PROTO_HDRS}")

set (DEBUG_FLAGS
    "-g")

set (OPT_FLAGS
    "-O0")

set (WARN_FLAGS
    "-Wall")

set(GCC_COMPILE_FLAGS
    "${DEBUG_FLAGS} ${OPT_FLAGS} ${WARN_FLAGS}")

set(CMAKE_CXX_FLAGS
    "${CMAKE_CXX_FLAGS} ${GCC_COMPILE_FLAGS}")

file(GLOB SOURCES           "src/*.cpp")
set(PROJECT_SOURCES         ${SOURCES} ${PROTO_SRCS} ${PROTO_HDRS})
set(PROJECT_INCLUDES        ${PROJECT_SOURCE_DIR}/include ${PROTOBUF_INCLUDE_DIRS} ${PROJECT_SOURCE_DIR}/build)
set(PROJECT_LIBRARIES       ${PROTOBUF_LIBRARIES})

include_directories(${PROJECT_INCLUDES})
add_executable(${PROJECT_NAME} ${PROJECT_SOURCES})
target_link_libraries(${PROJECT_NAME} PUBLIC ${PROJECT_LIBRARIES})

set_property(DIRECTORY PROPERTY ADDITIONAL_CLEAN_FILES
    "${PROJECT_SOURCE_DIR}/build/CMakeFiles"
    "${PROJECT_SOURCE_DIR}/build/CMakeCache.txt"
    "${PROJECT_SOURCE_DIR}/build/Makefile"
    "${PROJECT_SOURCE_DIR}/build/cmake_install.cmake"
    "${PROJECT_SOURCE_DIR}/build/lib"
    # "${PROJECT_SOURCE_DIR}/build/compile_commands.json"
)

add_custom_target("run"
    COMMENT "Running Program"
    DEPENDS ${PROJECT_NAME}
    WORKING_DIRECTORY ${PROJECT_SOURCE_DIR}
    COMMAND ./bin/${PROJECT_NAME}
)

add_custom_target("debug"
    COMMENT "Debugging with GDB"
    DEPENDS ${PROJECT_NAME}
    WORKING_DIRECTORY ${PROJECT_SOURCE_DIR}
    COMMAND gdb -tui -x ./GDBCOMMANDS --args ./bin/${PROJECT_NAME}
)
