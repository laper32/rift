cmake_minimum_required(VERSION 3.21 FATAL_ERROR)

if(POLICY CMP0092)
    cmake_policy(SET CMP0092 NEW) # Don't add -W3 warning level by default.
endif()

project(rift.engine VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Debug)
endif()

if(UNIX AND NOT APPLE)
    set(LINUX TRUE)
endif()

message(VERBOSE "*")
message(VERBOSE "* ${PROJECT_NAME} v${PROJECT_VERSION} (${CMAKE_BUILD_TYPE})")
message(VERBOSE "* Copyright (c) 2023 - Present laper32")
message(VERBOSE "*")

add_library(${PROJECT_NAME} SHARED 
    src/Main.cpp
    src/rift/Main.cpp
    src/rift/bridge/InteropService.cpp
    src/rift/bridge/natives/CoreNatives.cpp
    src/rift/coreclr/CoreCLR.cpp
    src/rift/fundamental/PrimitivePath.cpp
)
target_include_directories(${PROJECT_NAME} PUBLIC ${PROJECT_SOURCE_DIR}/include)

target_include_directories(${PROJECT_NAME} PRIVATE ${PROJECT_SOURCE_DIR}/src)


include(GenerateExportHeader)

generate_export_header(${PROJECT_NAME}
        BASE_NAME RIFT
        EXPORT_MACRO_NAME RIFT_API
        NO_EXPORT_MACRO_NAME RIFT_PRIVATE
        EXPORT_FILE_NAME ${CMAKE_BINARY_DIR}/exports/${PROJECT_NAME}.export.h
        STATIC_DEFINE RIFT_STATIC
)
target_include_directories(${PROJECT_NAME} PUBLIC ${CMAKE_BINARY_DIR}/exports)