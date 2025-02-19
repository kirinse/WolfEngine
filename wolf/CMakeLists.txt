#cmake . -B build -DCMAKE_TOOLCHAIN_FILE=$ANDROID_NDK_HOME/build/cmake/android.toolchain.cmake -DANDROID_ABI=armeabi-v7a -DANDROID_NDK=$ANDROID_NDK_HOME -DANDROID_PLATFORM=android-21 -DCMAKE_ANDROID_ARCH_ABI=armeabi-v7a -DCMAKE_ANDROID_NDK=$ANDROID_NDK_HOME -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DCMAKE_SYSTEM_NAME=Android -DCMAKE_SYSTEM_VERSION=21 -DCMAKE_BUILD_TYPE=Debug -GNinja
cmake_minimum_required(VERSION 3.22...3.23)

# set the name of the projects
project(wolf)
set(TEST_PROJECT_NAME "${PROJECT_NAME}_tests")
message("CXX Compiler ID is ${CMAKE_CXX_COMPILER_ID}")

# set the options and enviroment variables
set(WEBRTC_SRC $ENV{WEBRTC_ROOT} CACHE STRING "path to the root folder of webrtc folder")

# check the OS
if(CMAKE_SIZEOF_VOID_P EQUAL 8)
    if (WIN32)
        set(WIN64 TRUE)
    endif()
endif()

if(UNIX AND NOT APPLE)
   set(LINUX TRUE)
endif()

if (MSVC AND NOT WIN64)
    message( FATAL_ERROR "Only Window 64 bit is supported" )
endif()

# set target os
if (WIN64)
    set(TARGET_OS "win")
    set(LIB_EXT "lib")
    set(SHARED_EXT "dll")
elseif(APPLE)
    set(TARGET_OS "mac")
    set(LIB_EXT "a")
    set(SHARED_EXT "dylib")
elseif(LINUX)
    set(TARGET_OS "linux")
    set(LIB_EXT "a")
    set(SHARED_EXT "so")
else()
    message( FATAL_ERROR "Unsuported OS, please open an issue at https://github.com/WolfEngine/WolfEngine" )
endif()

# required packages
find_package(Git REQUIRED)
if (NOT EMSCRIPTEN)
    find_package (Threads)
endif()
if (LINUX)
    find_package(Wget REQUIRED)
    message("make sure libtool was installed.(apt-get install libtool-bin)")
endif()

# use folders
set_property(GLOBAL PROPERTY USE_FOLDERS ON)

# FetchContent for cloning repositories, avaiable since CMAKE 3.11
include(FetchContent)

# CMAKE GUI Options
option(WOLF_MEDIA_FFMPEG "Enable ffmpeg encoding and decoding" OFF)
option(WOLF_MEDIA_IMAGE "Enable encode/decode image formats" OFF)
option(WOLF_MEDIA_SCREEN_CAPTURE "Enable screen capture" OFF)
option(WOLF_STREAM_BORINGSLL "Enable boringSSL" OFF)
option(WOLF_STREAM_HTTP_SERVER "Enable http server" OFF)
option(WOLF_STREAM_QUIC "Enable QUIC" OFF)
option(WOLF_STREAM_RIST "Enable RIST streaming protocol" OFF)
option(WOLF_STREAM_RUDP "Enable reliable & unreliable stream over UDP" OFF)
option(WOLF_STREAM_TCP  "Enable TCP protocol" OFF)
option(WOLF_STREAM_WEBRTC "Enable webRTC" OFF)
option(WOLF_STREAM_WEBSOCKET_CLIENT "Enable websocket client" OFF)
option(WOLF_STREAM_WEBSOCKET_SERVER "Enable websocket server" OFF)
option(WOLF_SYSTEM_LOG "Enable log" OFF)
option(WOLF_SYSTEM_LZ4 "Enable lz4 for compression" ON)
option(WOLF_SYSTEM_LZMA "Enable lzma for compression" ON)
option(WOLF_SYSTEM_MIMALLOC "Enable Microsoft's mimalloc memory allocator" OFF)
option(WOLF_SYSTEM_STACKTRACE "Enable stacktrace" OFF)
option(WOLF_SYSTEM_ZLIB "Enable zlib for compression" ON)
option(WOLF_TESTS "Enable tests" OFF)

# set type of library
set(LIBRARY_TYPE "STATIC" CACHE STRING "Library type")


#option(WOLF_SYSTEM_FIBER "Enable boost fiber" OFF)
# option(WOLF_RENDER "Enable cross platform render based on BGFX" OFF)

# stream modules
# option(WOLF_STREAM_GRPC "Enable gRPC connection" OFF)
# option(WOLF_STREAM_KAFKA_CLIENT "Enable Kafka API connector" OFF)

# option(WOLF_STREAM_RTSP_CLIENT "Enable RTSP for client" OFF)
# option(WOLF_STREAM_RTSP_SERVER "Enable RTSP for server" OFF)
#option(WOLF_STREAM_HTTP_WS "Enable http 1.1/3 and websocket protocols" OFF)
#option(WOLF_STREAM_WEBRTC "Enable native webRTC" OFF)

# system modules
# option(WOLF_SYSTEM_AVX2 "Enable avx2" OFF)
# option(WOLF_SYSTEM_AVX512 "Enable avx512" OFF)
# option(WOLF_SYSTEM_BROTLI "Enable brotli compression" OFF)
# option(WOLF_SYSTEM_LUA "Enable lua scripting" OFF)
# option(WOLF_SYSTEM_LZ4 "Enable LZ4 Compression" OFF)
# option(WOLF_SYSTEM_LZMA "Enable LZMA Compression" OFF)
# option(WOLF_SYSTEM_RAFT "Enable raft algorithm" OFF)

# set C/CXX ios standards
set(CMAKE_C_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS ON)
set(CMAKE_THREAD_PREFER_PTHREAD TRUE)
set(THREADS_PREFER_PTHREAD_FLAG TRUE)

set(CMAKE_BUILD_TYPE "${default_build_type}" CACHE STRING "Choose the type of build." FORCE)
if ("${CMAKE_BUILD_TYPE}" STREQUAL "")
  set(CMAKE_BUILD_TYPE "Debug")
endif()
set_property(CACHE CMAKE_BUILD_TYPE PROPERTY STRINGS "Debug" "Release" "MinSizeRel" "RelWithDebInfo")

set(CMAKE_CXX_STANDARD 23)
if (MSVC)
set(CMAKE_CXX_FLAGS "/EHsc /W3")
else()
set(CMAKE_CXX_FLAGS "-std=c++2b -fexceptions")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")
endif()

#define includes, libs and srcs
set(INCLUDES ${CMAKE_CURRENT_SOURCE_DIR})
set(LIBS)
set(SRCS)

if (NOT EMSCRIPTEN)
    # currently threads was not supported with WASM
    list(APPEND LIBS Threads::Threads)
endif()

# include cmakes
include(cmake/boost.cmake)
include(cmake/system.cmake)
include(cmake/stream.cmake)

# fetch catch2 for tests
if (WOLF_TESTS)
    message("fetching https://github.com/catchorg/Catch2.git")
    FetchContent_Declare(
    catch2
    GIT_REPOSITORY https://github.com/catchorg/Catch2.git
    GIT_TAG        v2.x
    )
    FetchContent_Populate(catch2)
    list(APPEND INCLUDES
        ${catch2_SOURCE_DIR}/single_include
    )
endif()

# enabling clang-tidy
# can be enabled with .CLANG-TIDY from Visual Studio Code
# https://devblogs.microsoft.com/cppblog/visual-studio-code-c-december-2021-update-clang-tidy/
# can be enabled with .CLANG-TIDY from Visual Studio
# https://devblogs.microsoft.com/cppblog/code-analysis-with-clang-tidy-in-visual-studio/
# set(CMAKE_CXX_CLANG_TIDY
#     clang-tidy;
#     -format-style=file;)  

# add definitions
if (MSVC)
    add_definitions(
        -EHsc
        -DNOMINMAX
        -DWIN64
        -DWIN32
    )
elseif(APPLE)
    add_definitions(-DNEED_XLOCALE_H=1)
endif()

if(CMAKE_BUILD_TYPE MATCHES Debug)
    add_definitions(-DDEBUG -D_DEBUG)
else()
    add_definitions(-DNDEBUG)
endif()

# setup Wolf definitions
get_cmake_property(_vars VARIABLES)
foreach (_var ${_vars})
    string(FIND ${_var} "WOLF_" out)
    if(("${out}" EQUAL 0) AND ("(${${_var}}" MATCHES ON))
        add_definitions("-D${_var}")
    endif()
endforeach()

# include sources
file(GLOB_RECURSE WOLF_SRCS
    "${CMAKE_CURRENT_SOURCE_DIR}/wolf.hpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/wolf.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/tbv.hpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/DISABLE_ANALYSIS_BEGIN"
    "${CMAKE_CURRENT_SOURCE_DIR}/DISABLE_ANALYSIS_END"
)
file(GLOB_RECURSE WOLF_CMAKES
    "${CMAKE_CURRENT_SOURCE_DIR}/cmake/*"
)

file(GLOB_RECURSE WOLF_SYSTEM_SRC
    "${CMAKE_CURRENT_SOURCE_DIR}/system/getopt.h"
    "${CMAKE_CURRENT_SOURCE_DIR}/system/w_trace.hpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/system/w_gametime.hpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/system/w_gametime.cpp"
    "${CMAKE_CURRENT_SOURCE_DIR}/system/test.hpp"
)

# includes
include_directories(${INCLUDES})

# add source codes
add_library(${PROJECT_NAME} ${LIBRARY_TYPE} ${SRCS}
    ${WOLF_SRCS}
    ${WOLF_CMAKES}
	${WOLF_STREAM_QUIC_SRC}
    ${WOLF_STREAM_WEBRTC_CAPTURER_SRC}
    ${WOLF_STREAM_WEBRTC_DATA_SRC}
    ${WOLF_STREAM_WEBRTC_INTERCEPTOR_SRC}
    ${WOLF_STREAM_WEBRTC_MEDIA_SRC}
    ${WOLF_STREAM_WEBRTC_PEER_SRC}
    ${WOLF_SYSTEM_SRC}
    ${WOLF_SYSTEM_IO_SRC}
    ${WOLF_SYSTEM_LOG_SRC}
)
		
#if (MSVC)
#    if(CMAKE_BUILD_TYPE MATCHES Debug)
#        set_property(TARGET ${PROJECT_NAME} PROPERTY MSVC_RUNTIME_LIBRARY "MultiThreadedDebugDLL")
#    else()
#        set_property(TARGET ${PROJECT_NAME} PROPERTY MSVC_RUNTIME_LIBRARY "MultiThreadedDLL")
#    endif()
#endif()

# link libraries
target_link_libraries(${PROJECT_NAME} PRIVATE ${LIBS})

# create source group
source_group("wolf" FILES ${WOLF_SRCS})
source_group("cmake" FILES ${WOLF_CMAKES})
source_group("stream/quic" FILES ${WOLF_STREAM_QUIC_SRC})
source_group("stream/rist" FILES ${WOLF_STREAM_RIST_SRC})
source_group("stream/webrtc/capturer" FILES ${WOLF_STREAM_WEBRTC_CAPTURER_SRC})
source_group("stream/webrtc/data" FILES ${WOLF_STREAM_WEBRTC_DATA_SRC})
source_group("stream/webrtc/interceptor" FILES ${WOLF_STREAM_WEBRTC_INTERCEPTOR_SRC})
source_group("stream/webrtc/media" FILES ${WOLF_STREAM_WEBRTC_MEDIA_SRC})
source_group("stream/webrtc/peer" FILES ${WOLF_STREAM_WEBRTC_PEER_SRC})
source_group("system" FILES ${WOLF_SYSTEM_SRC})
source_group("system/io" FILES ${WOLF_SYSTEM_IO_SRC})
source_group("system/log" FILES ${WOLF_SYSTEM_LOG_SRC})

# add compile options
if (NOT WIN32)
    target_compile_options(${PROJECT_NAME} PRIVATE -std=c++2b -fPIC)
endif()

# build tests
if (WOLF_TESTS)
    add_executable (${TEST_PROJECT_NAME} tests.cpp)
    link_directories(${CMAKE_BINARY_DIR})
    target_link_libraries(${TEST_PROJECT_NAME} PRIVATE ${PROJECT_NAME})
    if (NOT EMSCRIPTEN)
        target_link_libraries(${TEST_PROJECT_NAME} PRIVATE Threads::Threads)
    endif()
    target_compile_options(${TEST_PROJECT_NAME} PRIVATE -std=c++2b)
endif()
