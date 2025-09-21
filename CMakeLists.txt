cmake_minimum_required(VERSION 3.24.0)
project(arma-gimp-plugin-cxx VERSION 0.1.0 LANGUAGES C CXX)

include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5 
)

FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)
corrosion_add_cxxbridge(aff_cxx_rust CRATE arma_file_formats_cxx MANIFEST_PATH arma-file-formats-cxx FILES lib.rs)


if(CMAKE_SYSTEM_NAME STREQUAL "Linux"
    OR (CMAKE_SYSTEM_NAME STREQUAL "Windows"
        AND (CMAKE_CXX_COMPILER_ID STREQUAL "GNU" OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
    )
)
    corrosion_add_target_local_rustflags(arma_file_formats_cxx "-Clink-arg=-fuse-ld=lld")
endif()

target_include_directories(aff_cxx_rust PRIVATE "include")

target_include_directories(aff_cxx_rust INTERFACE "include")
