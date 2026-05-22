$env:RUST_LOG = "warn"
$ROOT = Get-Location
$JOBS = if ($env:JOBS) { $env:JOBS } else { $env:NUMBER_OF_PROCESSORS }

Write-Host "[0/4] Cleaning previous build artifacts..."
Remove-Item "$ROOT/.gen-tmp", "$ROOT/svd", "$ROOT/html" -Recurse -Force -ErrorAction Ignore
# Clean generated chip dirs and generic.rs but keep hand-maintained lib.rs
Get-ChildItem "$ROOT/src" -Directory | Remove-Item -Recurse -Force
Remove-Item "$ROOT/src/generic.rs" -Force -ErrorAction Ignore
New-Item -ItemType Directory -Path . -Name svd -ErrorAction Ignore | Out-Null
New-Item -ItemType Directory -Path . -Name src -ErrorAction Ignore | Out-Null

# [1/4] Extract SVDs from PACKs
Write-Host "[1/4] Extracting SVDs from PACKs..."
Get-ChildItem ./PACKs/Keil5/ -Filter *.pack |
Foreach-Object -Parallel {
    $pack = $_
    $tmp = Join-Path $using:ROOT ".gen-tmp" $pack.BaseName
    New-Item -ItemType Directory -Path $tmp -Force | Out-Null
    Expand-Archive $pack -DestinationPath $tmp
    $svdDir = Join-Path $tmp "SVD"
    if (Test-Path $svdDir) {
        Get-ChildItem $svdDir -Filter *.svd |
        Foreach-Object {
            $name = $_.Name -replace 'xx_v2', ''
            $name = $name.ToLower()
            Copy-Item $_ -Destination (Join-Path $using:ROOT "svd" $name)
        }
    }
}

# [2/4] Patch SVDs
Write-Host "[2/4] Patching SVDs (svdtools)..."
Get-ChildItem ./patches/ -Filter *.yaml |
Foreach-Object -Parallel {
    svdtools patch $_
}

# [3/4] svd2rust + form per chip
Write-Host "[3/4] Running svd2rust + form per chip..."
Get-ChildItem ./svd/ -Filter *.svd.patched |
Foreach-Object -Parallel {
    $ErrorActionPreference = "Stop"
    $svd = $_
    $dirName = $svd.BaseName -replace '\.svd$', ''
    $dirName = $dirName.ToLower()
    $svd_name = "$dirName.svd"

    $work = Join-Path $using:ROOT ".gen-tmp" $dirName
    New-Item -ItemType Directory -Path $work -Force | Out-Null
    Copy-Item $svd -Destination (Join-Path $work $svd_name)
    Push-Location $work

    cargo init -q
    svd2rust -m -g -s `
        --edition 2024 `
        --reexport-core-peripherals --reexport-interrupt `
        -f peripheral::c: -f peripheral_singleton::c: -f register::c: -f register_spec::c:_SPEC -f field_reader::c:_R -f field_writer::c:_W -f interrupt::c: -f enum_name::c:_A -f enum_value::p: `
        --max_cluster_size `
        --atomics --atomics_feature atomics `
        --impl_debug --impl_defmt defmt `
        -l warn -i $svd_name
    if ($LASTEXITCODE -ne 0) {
        Pop-Location
        Write-Error "svd2rust failed for $dirName"
        exit 1
    }

    form -i mod.rs -o src/
    cargo fmt
    Copy-Item $svd_name -Destination src/
    Copy-Item device.x -Destination src/
    Rename-Item -Path src/lib.rs -NewName mod.rs

    Pop-Location

    $target = Join-Path $using:ROOT "src" $dirName
    Copy-Item (Join-Path $work "generic.rs") -Destination (Join-Path $using:ROOT "src" "generic.rs")
    Copy-Item -Path (Join-Path $work "src") -Destination $target -Recurse
}

# [4/4] HTML docs
Write-Host "[4/4] Generating HTML docs..."
$patched_svds = Get-ChildItem ./svd/ -Filter *.svd.patched
$orig_svds = Get-ChildItem ./svd/ -Filter *.svd | Where-Object { $_.Name -notlike '*.patched' }
New-Item -ItemType Directory -Path html, html/original -Force | Out-Null
svdtools html ./html $patched_svds | Out-Null
svdtools html ./html/original $orig_svds | Out-Null
svdtools htmlcompare ./html/comparison $patched_svds | Out-Null
svdtools htmlcompare ./html/original/comparison $orig_svds | Out-Null
$index = "./html/original/index.html"
(Get-Content $index).replace('comparisons.html', 'comparison/index.html') | Set-Content $index
$index = "./html/index.html"
(Get-Content $index).replace('comparisons.html', 'comparison/index.html') | Set-Content $index

Write-Host "Done."
