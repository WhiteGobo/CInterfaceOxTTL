include(CTest)

FetchContent_Declare(
	BasicRDFGraphComparator
	GIT_REPOSITORY https://github.com/WhiteGobo/BasicRDFGraphComparator.git
	GIT_TAG 8b244ca1826c024a641d98f0bb52e3c649e79972
	FIND_PACKAGE_ARGS
)
FetchContent_MakeAvailable(BasicRDFGraphComparator)

FetchContent_Declare(
	NQuadsRDF
	GIT_REPOSITORY https://github.com/WhiteGobo/NQuadsRDF.git
	#GIT_TAG 7f424c76698a0df4b500d30a5917eaa2e038e65a
	FIND_PACKAGE_ARGS
)
FetchContent_MakeAvailable(NQuadsRDF)

add_executable(CInterfaceOxTTL_testdriver_toRdfTest
	testdriverA.c
)

target_link_libraries(CInterfaceOxTTL_testdriver_toRdfTest PRIVATE
	CInterfaceOxTTL
	NQuadsRDF
	BasicRDFGraphComparator
)

include(config_tests.cmake)
