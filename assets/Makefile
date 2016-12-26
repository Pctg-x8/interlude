# Makefile for GLSL Shader Sources

CC_GLSL = $(SHADERC_BUILD_DIR)/glslc/glslc
.SUFFIXES: .vert .frag .geom .spv
VERT_OUTS = $(patsubst %.vert,%.spv,$(shell find . -type f -name '*.vert'))
FRAG_OUTS = $(patsubst %.frag,%.spv,$(shell find . -type f -name '*.frag'))
GEOM_OUTS = $(patsubst %.geom,%.spv,$(shell find . -type f -name '*.geom'))
ALL_OUTS = $(VERT_OUTS) $(FRAG_OUTS) $(GEOM_OUTS)

all: $(ALL_OUTS)

clean:
	rm -r **/*.spv

.vert.spv:
	$(CC_GLSL) $< -o $@

.frag.spv:
	$(CC_GLSL) $< -o $@

.geom.spv:
	$(CC_GLSL) $< -o $@
