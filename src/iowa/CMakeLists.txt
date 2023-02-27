cmake_minimum_required(VERSION 3.25)

project(iowa)

# message($ENV{IOWA_DIR})

include($ENV{IOWA_DIR}/src/iowa.cmake)

# message(${IOWA_CLIENT_SOURCES})
# message(${IOWA_CLIENT_HEADERS})

add_library(${PROJECT_NAME} STATIC
            ${CMAKE_CURRENT_LIST_DIR}/iowa_config.h
            $ENV{IOWA_DIR}/samples/abstraction_layer/core_abstraction.c
            $ENV{IOWA_DIR}/samples/abstraction_layer/connection_abstraction.c
            $ENV{IOWA_DIR}/samples/abstraction_layer/mutex_abstraction.c
            ${IOWA_CLIENT_SOURCES}
            ${IOWA_CLIENT_HEADERS})

target_include_directories(${PROJECT_NAME} PRIVATE
            ${IOWA_INCLUDE_DIR}
            ${CMAKE_CURRENT_LIST_DIR})