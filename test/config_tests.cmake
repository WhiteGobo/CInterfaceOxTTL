include(tools.cmake)

set(basepath "${CMAKE_CURRENT_SOURCE_DIR}/data")
read_manifest("${basepath}/manifest.json" testarray length)

set(basename "CInterfaceOxTTL::")
math(EXPR range "${length} - 1")
foreach(x RANGE 0 ${range})
	set(extras "")
	string(JSON data GET ${testarray} ${x})
	string(JSON testsuffix GET ${data} name)
	string(JSON premise GET ${data} premise)
	string(JSON query GET ${data} query)
	string(JSON testtype GET ${data} "@type")
	if (testtype MATCHES "NegativeEntailment")
		list(APPEND extras "--expected-failure")
	endif()
	if (testtype MATCHES "DatasetTest")
		list(APPEND extras "--use-trig")
	endif()

	set(testname "${basename}${testsuffix}")
	add_test(NAME "${testname}"
		COMMAND CInterfaceOxTTL_testdriver_toRdfTest
		ARGS
		"--premise" "${basepath}/${premise}"
		"--out" "${basepath}/${query}"
		${extras}
	)
	set_property(TEST "${testname}" PROPERTY
		LABELS "CInterfaceOxTTL" "${testtype}")

endforeach()
