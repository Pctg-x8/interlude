// Interlude Extras

use super::*;

impl VkAttachmentReference
{
	pub fn color(index: u32) -> Self { VkAttachmentReference(index, VkImageLayout::ColorAttachmentOptimal) }
	pub fn input(index: u32) -> Self { VkAttachmentReference(index, VkImageLayout::ShaderReadOnlyOptimal) }
}