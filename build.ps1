$ErrorActionPreference = 'Stop'
$pluginRoot = Join-Path $PSScriptRoot 'com.martijn.opendeck.davinciresolve.sdPlugin'
$target = 'x86_64-pc-windows-msvc'

cargo build --release --manifest-path (Join-Path $PSScriptRoot 'src/Cargo.toml') --target $target
if ($LASTEXITCODE -ne 0) { throw 'Cargo build failed.' }

$binaryDir = Join-Path $pluginRoot "$target/bin"
New-Item -ItemType Directory -Force -Path $binaryDir | Out-Null
New-Item -ItemType Directory -Force -Path $pluginRoot | Out-Null
Copy-Item (Join-Path $PSScriptRoot 'assets/manifest.json') (Join-Path $pluginRoot 'manifest.json') -Force
Copy-Item (Join-Path $PSScriptRoot 'assets/icons') $pluginRoot -Recurse -Force
Copy-Item (Join-Path $PSScriptRoot 'LICENSE') $pluginRoot -Force
Copy-Item (Join-Path $PSScriptRoot 'README.md') $pluginRoot -Force
Copy-Item (Join-Path $PSScriptRoot "src/target/$target/release/opendeck-davinci-resolve.exe") (Join-Path $binaryDir 'opendeck-davinci-resolve.exe') -Force
$archive = Join-Path $PSScriptRoot 'DaVinciResolve-OpenDeck.sdPlugin.zip'
Compress-Archive -Path $pluginRoot -DestinationPath $archive -Force
Write-Host "Installable plugin archive: $archive"
