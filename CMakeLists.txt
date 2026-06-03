cmake_minimum_required(VERSION 3.25)

project(CInterfaceOxTTL VERSION 0.1.0 LANGUAGES C)

add_subdirectory(src)

## Testing

enable_testing()
add_subdirectory(test)
