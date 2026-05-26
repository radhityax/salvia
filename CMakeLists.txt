cmake_minimum_required(VERSION 3.20)
project(salvia VERSION 0.1.0 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(FetchContent)
FetchContent_Declare(
    cmark
    GIT_REPOSITORY https://github.com/commonmark/cmark.git
    GIT_TAG master
)
FetchContent_MakeAvailable(cmark)

add_executable(salvia src/main.cpp)
target_link_libraries(salvia PRIVATE cmark)

