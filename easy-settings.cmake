set(CAMKES_APP "system" CACHE STRING "The one and only CAmkES application in this project")
#set(CAPDL_LOADER_APP "capdl-loader-app" CACHE STRING "")
set(CAPDL_LOADER_APP "cantrip-os-rootserver" CACHE STRING "")

set(PLATFORM "sparrow" CACHE STRING "The one and only seL4 platform for Sparrow")
set(RUST_TARGET "riscv32imac-unknown-none-elf" CACHE STRING "Rust target_arch")
set(KernelSel4Arch "riscv32" CACHE STRING "Specifies 32-bit branch of the seL4 spike platform")
set(KernelIsMCS ON CACHE BOOL "Enable seL4 MCS support")
set(KernelPrinting ON CACHE BOOL "Enable seL4 console output support")

set(LibUtilsDefaultZfLogLevel 5 CACHE STRING "seL4 internal logging level (0-5).")
set(SIMULATION ON CACHE BOOL "Whether to build simulate script")
set(RELEASE OFF CACHE BOOL "Performance optimized build")
set(UseRiscVBBL OFF CACHE BOOL "Whether to use bbl")
