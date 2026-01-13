# visual-cd
Interactive command-line tool to explore and change directories using a tree view (Rust + Ratatui)

On Windows, add function in PowerShell:

function ccd {
    $startDir = (Get-Location -PSProvider FileSystem).ProviderPath

    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = "vcd.exe"
    $psi.Arguments = "`"$startDir`""   
    $psi.UseShellExecute = $false
    $psi.RedirectStandardError = $true
    $psi.RedirectStandardOutput = $false  

    $p = [System.Diagnostics.Process]::Start($psi)
    $stderr = $p.StandardError.ReadToEnd()
    $p.WaitForExit()

    $status = $p.ExitCode
    $dir = $stderr.Trim()

    switch ($status) {
        0 { if ($dir) { Set-Location -LiteralPath $dir } }
        1 { } # nic nie robimy
        default { if ($dir) { Write-Error $dir } else { Write-Error "vcd failed with exit code $status" } }
    }
}
