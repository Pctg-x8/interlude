fn main()
{
	// Adding Vulkan Library Path
	let mut vksdk_base = std::env::var("VULKAN_SDK").map(std::path::PathBuf::from)
		.expect("required Vulkan SDK Base Path as $Env:VULKAN_SDK");
	vksdk_base.push("Lib");
	println!("cargo:rustc-link-search={}", vksdk_base.to_string_lossy());
}