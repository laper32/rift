cmake_minimum_required(VERSION 3.28)
project(rift)
set(CMAKE_CXX_STANDARD 20)
add_subdirectory(third_party)
add_executable(rift
	main.cpp
	lib/core/Application.cpp
	lib/core/Application.cpp
	lib/core/ModuleInstance.cpp
	lib/core/ModuleDescriptor.h)
target_include_directories(rift PUBLIC ${CMAKE_CURRENT_LIST_DIR}/include)
target_link_libraries(rift PRIVATE v8)
