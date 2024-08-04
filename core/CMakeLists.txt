cmake_minimum_required(VERSION 3.28)
project(rift)
set(CMAKE_CXX_STANDARD 20)
add_subdirectory(third_party)
add_library(rift-core SHARED
	src/Startup.cpp
	src/core/Application.cpp
	src/core/Application.cpp
	src/core/ModuleInstance.cpp
	src/core/ModuleDescriptor.h)
target_include_directories(rift-core PUBLIC ${CMAKE_CURRENT_LIST_DIR}/include)
target_link_libraries(rift-core PRIVATE wee8)
set_property(TARGET rift-core PROPERTY
             MSVC_RUNTIME_LIBRARY "MultiThreaded")