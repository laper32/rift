cmake_minimum_required(VERSION 3.28)

project(sample)

set(CMAKE_CXX_STANDARD 20)

add_library(sample SHARED main.cpp)

target_include_directories(sample PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})
target_link_libraries(sample PRIVATE engine)

add_library(engine INTERFACE)
target_link_libraries(engine INTERFACE ${CMAKE_CURRENT_LIST_DIR}/../../target/release/engine.dll.lib)