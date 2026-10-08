$ErrorActionPreference = 'Stop'
foreach ($name in @('RUSTUP_HOME', 'CARGO_HOME')) {
    $value = [Environment]::GetEnvironmentVariable($name, 'User')
    if ($value) { [Environment]::SetEnvironmentVariable($name, $value, 'Process') }
}
if ($env:CARGO_HOME) { $env:Path = (Join-Path $env:CARGO_HOME 'bin') + ';' + $env:Path }
Push-Location $PSScriptRoot
try {
    cargo build --release
    if ($LASTEXITCODE -ne 0) { throw 'Rust build failed' }
    $targetRoot = if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $PSScriptRoot 'target' }
    Copy-Item -LiteralPath (Join-Path $targetRoot 'release\desktop-ink.exe') -Destination (Join-Path $PSScriptRoot 'DesktopInk.exe') -Force
    Write-Output 'Built DesktopInk.exe'
} finally { Pop-Location }
