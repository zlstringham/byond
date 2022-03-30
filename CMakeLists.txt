cmake_minimum_required(VERSION 3.12.4)

project(byondrs VERSION 0.1.0)

if (CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
  include(CTest)
endif ()

add_subdirectory(c-ffi)
