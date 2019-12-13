renderer_sources = ['renderer.cc']

librenderer = shared_library('renderer', renderer_sources, install: true,
			     include_directories: ['.', '../utils'], dependencies: libutils)
librenderer = declare_dependency(link_with: librenderer, include_directories: ['.'])
