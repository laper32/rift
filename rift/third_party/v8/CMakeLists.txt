# We use puerTS's precompiled V8 binaries.
# See: https://puerts.github.io/docs/puerts/unreal/install/
# Current Version: 10.6.194
add_library(v8_libplatform INTERFACE)
target_include_directories(v8_libplatform INTERFACE
	${CMAKE_CURRENT_LIST_DIR}/Inc
)
target_link_libraries(v8_libplatform INTERFACE ${CMAKE_CURRENT_LIST_DIR}/Lib/Win64DLL/v8_libplatform.dll.lib)


add_library(v8 INTERFACE)
target_include_directories(v8 INTERFACE
	${CMAKE_CURRENT_LIST_DIR}/Inc
)
target_link_libraries(v8 INTERFACE 
	${CMAKE_CURRENT_LIST_DIR}/Lib/Win64DLL/v8.dll.lib
	v8_libplatform
)
