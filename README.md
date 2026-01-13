# visual-cd
Interactive command-line tool to explore and change directories using a tree view (Rust + Ratatui)

On Windows, add function in PowerShell:

```
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
```

On Linux, add function in .bashrc

```
ccd() {
    dir=$(vcd 2>&1 >/dev/tty)
    status=$?

    case $status in
        0)
            cd "$dir"
            ;;
        1)
            ;;
        *)
            echo "$dir" >&2
            ;;
    esac
}
```

┌Visual cd───────────────────────────────────────────────────────────────────────────────┐
│📂 /                                                                                    ▲
│├───📁 bin.usr-is-merged                                                                █
│├───📁 boot                                                                             █
│├───📁 cdrom                                                                            █
│├───📁 dev                                                                              █
│├───📁 disks                                                                            ║
│├───📂 etc                                                                              ║
││     ├───📁 alsa                                                                         ║
││     ├───📂 alternatives                                                                 ║
││     ├───📂 apache2                                                                      ║
││     │     ├───📁 conf-available                                                           ║
││     │     └───📁 mods-available                                                           ║
││     ├───📁 apm                                                                          ║
││     ├───📂 apparmor                                                                     ║
││     ├───📁 apparmor.d                                                                   ║
││     ├───📁 apport                                                                       ║
││     ├───📂 apt                                                                          ║
││     │     ├───📁 apt.conf.d                                                               ║
││     │     ├───📁 auth.conf.d                                                              ║
││     │     ├───📁 keyrings                                                                 ║
││     │     ├───📁 preferences.d                                                            ║
││     │     ├───📁 preferences.d.save                                                       ║
││     │     ├───📁 sources.list.d                                                           ║
││     │     └───📁 trusted.gpg.d                                                            ║
││     ├───📁 avahi                                                                        ║
││     ├───📁 bash_completion.d                                                            ║
││     ├───📁 binfmt.d                                                                     ▼
└/etc/apt/apt.conf.d─────────────────────────────────────────────────────────────────────┘

