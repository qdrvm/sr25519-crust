include(GNUInstallDirs)

# Compute the installation prefix relative to this file.
get_filename_component(_IMPORT_PREFIX "${CMAKE_CURRENT_LIST_DIR}/../../../" ABSOLUTE)

if (CMAKE_BUILD_TYPE STREQUAL "Release" OR CMAKE_BUILD_TYPE STREQUAL "RelWithDebInfo")
  set(build_type "release")
else ()
  set(build_type "debug")
endif ()
message(STATUS "CMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE}, using ${build_type} config.")

set(shared_lib_name ${CMAKE_SHARED_LIBRARY_PREFIX}schnorrkel_crust${CMAKE_SHARED_LIBRARY_SUFFIX})
set(shared_lib_path ${_IMPORT_PREFIX}/${CMAKE_INSTALL_LIBDIR}/${build_type}/${shared_lib_name})
set(static_lib_name ${CMAKE_STATIC_LIBRARY_PREFIX}schnorrkel_crust${CMAKE_STATIC_LIBRARY_SUFFIX})
set(static_lib_path ${_IMPORT_PREFIX}/${CMAKE_INSTALL_LIBDIR}/${build_type}/${static_lib_name})
if(EXISTS ${shared_lib_path})
    set(lib_path ${shared_lib_path})
elseif(EXISTS ${static_lib_path})
    set(lib_path ${static_lib_path})
else()
    message(FATAL_ERROR "schnorrkel_crust library (${shared_lib_name} or ${static_lib_name}) not found in ${_IMPORT_PREFIX}/${CMAKE_INSTALL_LIBDIR}!")
endif()

set(include_path ${_IMPORT_PREFIX}/${CMAKE_INSTALL_INCLUDEDIR}/schnorrkel)
if(NOT EXISTS ${include_path})
    message(FATAL_ERROR "schnorrkel_crust headers not found in ${include_path}!")
endif()

if(NOT TARGET schnorrkel_crust::schnorrkel_crust)
    add_library(schnorrkel_crust::schnorrkel_crust STATIC IMPORTED GLOBAL)

    if(EXISTS ${_IMPORT_PREFIX}/${CMAKE_INSTALL_LIBDIR}/${static_lib_name})
        if (APPLE)
            # on apple we need to link Security
            find_library(Security Security)
            find_package_handle_standard_args(schnorrkel_crust
                REQUIRED_VARS Security
                )
            set_target_properties(schnorrkel_crust::schnorrkel_crust PROPERTIES
                INTERFACE_LINK_LIBRARIES ${Security}
                )
        elseif (UNIX)
            # on Linux we need to link pthread
            target_link_libraries(schnorrkel_crust::schnorrkel_crust INTERFACE
                pthread
                -Wl,--no-as-needed
                dl
                )
        else ()
            message(ERROR "You've built static lib, it may not link on this platform. Come here and fix.")
        endif ()
    endif()


    set_target_properties(schnorrkel_crust::schnorrkel_crust PROPERTIES
        INTERFACE_INCLUDE_DIRECTORIES ${include_path}
        IMPORTED_LOCATION ${lib_path}
        )
endif()

unset(shared_lib_name)
unset(static_lib_name)
unset(shared_lib_path)
unset(static_lib_path)
unset(lib_path)
unset(include_path)

check_required_components(schnorrkel_crust)
