# We use puerTS's precompiled V8 binaries.
# See: https://puerts.github.io/docs/puerts/unreal/install/
# Current Version: 10.6.194


add_library(wee8 INTERFACE)
target_include_directories(wee8 INTERFACE
	${CMAKE_CURRENT_LIST_DIR}/Inc
)
target_link_libraries(wee8 INTERFACE
	${CMAKE_CURRENT_LIST_DIR}/Lib/Win64/wee8.lib
	dbghelp winmm shlwapi
)