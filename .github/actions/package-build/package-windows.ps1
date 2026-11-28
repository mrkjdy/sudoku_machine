param(
    [Parameter(Mandatory)]
    [string]$PackageName,
    [Parameter(Mandatory)]
    [string]$BinaryName,
    [Parameter(Mandatory)]
    [string]$BuildTarget
)

$ErrorActionPreference = 'Stop'

New-Item -ItemType Directory -Path windows | Out-Null
Copy-Item -LiteralPath "target/$BuildTarget/release/$BinaryName.exe" -Destination windows/
Copy-Item -LiteralPath assets -Destination windows/ -Recurse
Compress-Archive -Path 'windows/*' -DestinationPath "$PackageName.zip"
