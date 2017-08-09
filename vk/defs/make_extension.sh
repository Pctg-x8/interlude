#!/bin/sh

# usage: make_extension [khr/ext/nv/amd/...] [extension-name] [spec-version]

extension_class=$1
extension_name=$2
spec_version=$3
spec_hdr=VK_${extension_class^^}
output_path=src/${extension_name}_${extension_class}.rs

echo "//! ${spec_hdr}_$extension_name extensions" >> $output_path
echo "" >> $output_path
echo "pub const ${spec_hdr}_${extension_name^^}_SPEC_VERSION: usize = $spec_version;" >> $output_path
echo "pub static ${spec_hdr}_${extension_name^^}_EXTENSION_NAME: &'static str = \"${spec_hdr}_${extension_name}\";" >> $output_path
echo "" >> $output_path
