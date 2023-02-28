#
# Copyright 2018, Data61, CSIRO (ABN 41 687 119 230)
#
# SPDX-License-Identifier: BSD-2-Clause
#

cmake_minimum_required(VERSION 3.7.2)

project(hellorust)

#include(rust)

includeGlobalComponents()
include(${CMAKE_CURRENT_LIST_DIR}/rust.cmake)

RustAddLibrary(
    main
    SOURCE_DIR
    ${CMAKE_CURRENT_LIST_DIR}/components/main
    LIB_FILENAME
    libmain.a
)

DeclareCAmkESComponent(main LIBS main)

DeclareCAmkESRootserver(hellorust.camkes)
#add_simulate_test([=[wait_for "Hello, world!!"]=])
