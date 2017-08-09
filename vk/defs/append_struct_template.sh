#!/bin/sh

# usage: append_struct_template [target-file] [struct-name] [pNext-constness]

target_file=$1
struct_name=$2
constness=$3

echo "#[repr(C)] #[derive(Debug, Clone, PartialEq, Eq)]" >> $target_file
echo "pub struct $struct_name" >> $target_file
echo "{" >> $target_file
echo "	pub sType: VkStructureType, pub pNext: *$constness c_void" >> $target_file
echo "}" >> $target_file
