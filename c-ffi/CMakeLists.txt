find_program(CARGO_COMMAND cargo REQUIRED)

set(BYONDRS_INCLUDE ${CMAKE_CURRENT_BINARY_DIR}/include)
set(BYONDRS_H ${BYONDRS_INCLUDE}/byondrs/byondrs.h)

add_custom_target(
  byondrs_rust ALL
  COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}
          ${CARGO_COMMAND} build $<$<CONFIG:Release>:--release>
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
  set_target_properties(
    byondrs
    PROPERTIES IMPORTED_LOCATION ${CMAKE_CURRENT_BINARY_DIR}/debug/byondrs.dll
               IMPORTED_LOCATION_RELEASE
               ${CMAKE_CURRENT_BINARY_DIR}/release/byondrs.dll
               IMPORTED_LOCATION_DEBUG
               ${CMAKE_CURRENT_BINARY_DIR}/debug/byondrs.dll
               IMPORTED_IMPLIB ${CMAKE_CURRENT_BINARY_DIR}/debug/byondrs.dll.lib
               IMPORTED_IMPLIB_DEBUG
               ${CMAKE_CURRENT_BINARY_DIR}/debug/byondrs.dll.lib
               IMPORTED_IMPLIB_RELEASE
               ${CMAKE_CURRENT_BINARY_DIR}/release/byondrs.dll.lib)
elseif (CMAKE_SYSTEM_NAME STREQUAL "Darwin")
  set_target_properties(
    byondrs
    PROPERTIES IMPORTED_LOCATION
               ${CMAKE_CURRENT_BINARY_DIR}/debug/libbyondrs.dylib
               IMPORTED_LOCATION_DEBUG
               ${CMAKE_CURRENT_BINARY_DIR}/debug/libbyondrs.dylib
               IMPORTED_LOCATION_RELEASE
               ${CMAKE_CURRENT_BINARY_DIR}/release/libbyondrs.dylib)
else ()
  set_target_properties(
    byondrs
    PROPERTIES IMPORTED_LOCATION ${CMAKE_CURRENT_BINARY_DIR}/debug/libbyondrs.so
               IMPORTED_LOCATION_DEBUG
               ${CMAKE_CURRENT_BINARY_DIR}/debug/libbyondrs.so
               IMPORTED_LOCATION_RELEASE
               ${CMAKE_CURRENT_BINARY_DIR}/release/libbyondrs.so)
endif ()

if (CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME AND BUILD_TESTING)
  add_test(
    NAME byondrs_test
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
endif ()
