include(FetchContent)

FetchContent_Declare(
	Corrosion
	GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
	GIT_TAG v0.6 # Optionally specify a commit hash, version tag or branch here
	FIND_PACKAGE_ARGS
)
FetchContent_MakeAvailable(Corrosion)


add_library(CInterfaceOxTTL_CHelperlib STATIC CInterfaceOxTTL_CHelperlib.c)

corrosion_import_crate(MANIFEST_PATH ./Cargo.toml)
corrosion_link_libraries(CInterfaceOxTTL CInterfaceOxTTL_CHelperlib)
target_sources(CInterfaceOxTTL INTERFACE
        FILE_SET HEADERS
        BASE_DIRS include
	FILES
	include/CInterfaceOxTTL.h
)

corrosion_install(TARGETS CInterfaceOxTTL EXPORT CInterfaceOxTTLtargets)
