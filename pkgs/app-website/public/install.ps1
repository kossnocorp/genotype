# Genotype installer
# irm https://genotype-lang.org/install.ps1 | iex

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$TagVersion = $env:VERSION
$GhRepoName = "kossnocorp/genotype"
$GhApiHeaders = @{
  Accept                 = "application/vnd.github+json"
  "X-GitHub-Api-Version" = "2026-03-10"
}

if (-not [Environment]::Is64BitProcess) {
  Write-Error "Genotype requires a 64-bit Windows."
  exit 1
}

$arch = if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") { "aarch64" } else { "x86_64" }
$target = "$arch-pc-windows-msvc"

$releaseUrl = if (-not [string]::IsNullOrWhiteSpace($TagVersion)) {
  "https://api.github.com/repos/$GhRepoName/releases/tags/$TagVersion"
}
else {
  "https://api.github.com/repos/$GhRepoName/releases/latest"
}

try {
  $release = Invoke-RestMethod -Uri $releaseUrl -Headers $GhApiHeaders -ErrorAction Stop
}
catch {
  Write-Error "Failed to fetch release metadata from GitHub: $releaseUrl"
  exit 1
}

$version = $release.tag_name
if ([string]::IsNullOrWhiteSpace($version)) {
  Write-Error "Failed to determine release version from GitHub response."
  exit 1
}

$binary = "gt-$version-$target.exe"
$asset = $release.assets | Where-Object { $_.name -eq $binary } | Select-Object -First 1
if ($null -eq $asset) {
  Write-Error "No release asset found for $binary"
  exit 1
}

$digest = $asset.digest
if ([string]::IsNullOrWhiteSpace($digest) -or -not $digest.StartsWith("sha256:")) {
  Write-Error "No SHA256 digest found for $binary"
  exit 1
}

$checksum = $digest.Substring(7).ToLowerInvariant()
$url = "https://github.com/$GhRepoName/releases/download/$version/$binary"
$installDir = Join-Path (Join-Path $env:USERPROFILE ".genotype") "bin"
$installPath = Join-Path $installDir "gt.exe"
$tmp = [System.IO.Path]::GetTempFileName() + ".exe"

try {
  Write-Output "Downloading Genotype $version ($target)..."
  if (Get-Command curl.exe -ErrorAction SilentlyContinue) {
    & curl.exe -fL --progress-bar -o $tmp $url
    if ($LASTEXITCODE -ne 0) {
      Write-Error "Failed to download binary from $url"
      exit 1
    }
  }
  else {
    $oldProgressPreference = $ProgressPreference
    $ProgressPreference = "SilentlyContinue"
    try {
      Invoke-WebRequest -Uri $url -OutFile $tmp -ErrorAction Stop
    }
    finally {
      $ProgressPreference = $oldProgressPreference
    }
  }

  Write-Host -NoNewline "Verifying checksum... "
  $actual = (Get-FileHash -Path $tmp -Algorithm SHA256).Hash.ToLowerInvariant()
  if ($actual -ne $checksum) {
    Write-Error "Checksum mismatch!`n  expected: $checksum`n  actual:   $actual"
    exit 1
  }

  Write-Output "ok"

  New-Item -ItemType Directory -Force -Path $installDir | Out-Null
  Move-Item -Force $tmp $installPath

  $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
  $pathEntries = if ([string]::IsNullOrWhiteSpace($userPath)) { @() } else { $userPath -split ";" }
  if ($pathEntries -notcontains $installDir) {
    $newUserPath = if ([string]::IsNullOrWhiteSpace($userPath)) { $installDir } else { "$userPath;$installDir" }
    [Environment]::SetEnvironmentVariable("PATH", $newUserPath, "User")
    Write-Output "Added $installDir to your PATH (restart your shell to take effect)."
  }

  Write-Output ""
  Write-Output "Genotype $version installed to $installPath"
}
finally {
  if (Test-Path $tmp) { Remove-Item -Force $tmp -ErrorAction SilentlyContinue }
}
