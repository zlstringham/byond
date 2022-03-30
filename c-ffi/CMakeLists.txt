find_program(CARGO cargo REQUIRED)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
  set(CARGO_CMD ${CARGO} build)
else ()
  set(CARGO_CMD ${CARGO} build --release)
endif ()

set(BYONDRS_INCLUDE ${CMAKE_CURRENT_BINARY_DIR}/include)
set(BYONDRS_H ${BYONDRS_INCLUDE}/byondrs/byondrs.h)

add_custom_target(
  byondrs_rust ALL
  COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}
          ${CARGO_CMD}
  BYPRODUCTS ${BYONDRS_H}
  COMMENT "Building byondrs rust library."
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

add_library(byondrs SHARED IMPORTED GLOBAL)
add_dependencies(byondrs byondrs_rust)

# Hack: target_include_directories() fails at configure time if the directory
# doesn't exist, but the custom target generates it at build time.
file(MAKE_DIRECTORY ${BYONDRS_INCLUDE})
target_include_directories(byondrs INTERFACE ${BYONDRS_INCLUDE})

# Hack: target_sources() fails at configure time if the file doesn't exist, but
# the custom target generates it at build time.
file(MAKE_DIRECTORY ${BYONDRS_INCLUDE}/byondrs)
file(TOUCH ${BYONDRS_H})
target_sources(byondrs INTERFACE ${BYONDRS_H})

# TODO: Verify this works on Windows.
if (CMAKE_SYSTEM_NAME STREQUAL "Windows")
  set_property(
    TARGET byondrs PROPERTY IMPORTED_LOCATION
                            ${CMAKE_CURRENT_BINARY_DIR}/release/byondrs.dll)
  set_property(
    TARGET byondrs PROPERTY IMPORTED_IMPLIB
                            ${CMAKE_CURRENT_BINARY_DIR}/release/byondrs.dll.lib)
  set_property(
    TARGET byondrs PROPERTY IMPORTED_LOCATION_DEBUG
                            ${CMAKE_CURRENT_BINARY_DIR}/debug/byondrs.dll)
  set_property(
    TARGET byondrs PROPERTY IMPORTED_IMPLIB_DEBUG
                            ${CMAKE_CURRENT_BINARY_DIR}/debug/byondrs.dll.lib)
elseif (CMAKE_SYSTEM_NAME STREQUAL "Darwin")
  set_property(
    TARGET byondrs
    PROPERTY IMPORTED_LOCATION
             ${CMAKE_CURRENT_BINARY_DIR}/release/libbyondrs.dylib)
  set_property(
    TARGET byondrs PROPERTY IMPORTED_LOCATION_DEBUG
                            ${CMAKE_CURRENT_BINARY_DIR}/debug/libbyondrs.dylib)
else ()
  set_property(
    TARGET byondrs PROPERTY IMPORTED_LOCATION
                            ${CMAKE_CURRENT_BINARY_DIR}/release/libbyondrs.so)
  set_property(
    TARGET byondrs PROPERTY IMPORTED_LOCATION_DEBUG
                            ${CMAKE_CURRENT_BINARY_DIR}/debug/libbyondrs.so)
endif ()

if (CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME AND BUILD_TESTING)
  add_test(
    NAME byondrs_test
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
endif ()
