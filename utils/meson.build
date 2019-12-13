util_sources = ['ppm.cc']

libutils = shared_library('utils', util_sources, install: true)
libutils = declare_dependency(link_with: libutils, include_directories: '.')

executable('test_ppm', 'test.cc', dependencies: libutils)
