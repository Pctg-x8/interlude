$Items = ((Get-ChildItem -Path $PSScriptRoot -Filter *.vert -Recurse) +
	(Get-ChildItem -Path . -Filter *.geom -Recurse) +
	(Get-ChildItem -Path . -Filter *.frag -Recurse)) | Select FullName

ForEach($Src in $Items)
{
	Write-Host ">> Processing" $Src.FullName "..."
	& "${Env:SHADERC_BUILD_DIR}\glslc\Release\glslc.exe" $Src.FullName -o ([io.path]::ChangeExtension($Src.FullName, "spv"))
}
