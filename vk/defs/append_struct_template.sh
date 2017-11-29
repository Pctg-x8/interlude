#!/bin/sh

# usage: append_struct_template [vendor-id] [extension-name] [struct-name] [pNext-constness]

vendor_id=${1,,}
extension_name=${2,,}
struct_name=$3
constness=$4
target_file=src/${extension_name}_${vendor_id}.rs
struct_name_comp=Vk${struct_name}${vendor_id^^}
stype=VK_STRUCTURE_TYPE_$(sed "s/[A-Z]/_&/g;s/^_//" <<< ${struct_name})_${vendor_id^^}

echo "#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]" >> $target_file
echo "pub struct $struct_name_comp" >> $target_file
echo "{" >> $target_file
echo "	pub sType: VkStructureType, pub pNext: *$constness c_void" >> $target_file
echo "}" >> $target_file
echo "impl Default for $struct_name_comp" >> $target_file
echo "{" >> $target_file
echo "	fn default() -> Self" >> $target_file
echo "	{" >> $target_file
echo "		$struct_name_comp" >> $target_file
echo "		{" >> $target_file
echo "			sType: ${stype^^}," >> $target_file
echo "			.. unsafe { std::mem::zeroed() }" >> $target_file
echo "		}" >> $target_file
echo "	}" >> $target_file
echo "}" >> $target_file
