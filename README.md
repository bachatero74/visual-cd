# visual-cd

**visual-cd** is a small terminal-based utility written in **Rust**, built using the **ratatui** TUI framework.
It lets you navigate the directory tree visually and change directories **without manually typing `cd`**.

The application works on both **Linux** and **Windows** terminals.

---

## Features

* Visual navigation of the directory tree
* Keyboard-driven interface
* No need to type or copy paths
* Built with **Rust** and **ratatui**
* Integrates cleanly with your shell (`bash`, PowerShell)
* Lightweight and fast

---

## Controls

* **Arrow keys** – navigate through directories
* **Any letter** – go to next item which name starts with this letter
* **Enter** – select the directory, exit the application, and change the current directory
* **Esc** – cancel the directory change and exit the application
  
---

## Preview

```text
┌Visual cd───────────────────────────────────────────────────────────────────────────────┐
│📂 /                                                                                    ▲
│├───📁 bin.usr-is-merged                                                                █
│├───📁 boot                                                                             █
│├───📁 cdrom                                                                            █
│├───📁 dev                                                                              █
│├───📁 disks                                                                            ║
│├───📂 etc                                                                              ║
││     ├───📁 alsa                                                                       ║
││     ├───📂 alternatives                                                               ║
││     ├───📂 apache2                                                                    ║
││     │     ├───📁 conf-available                                                       ║
││     │     └───📁 mods-available                                                       ║
││     ├───📁 apm                                                                        ║
││     ├───📂 apparmor                                                                   ║
││     ├───📁 apparmor.d                                                                 ║
││     ├───📁 apport                                                                     ║
││     ├───📂 apt                                                                        ║
││     │     ├───📁 apt.conf.d                                                           ║
││     │     ├───📁 auth.conf.d                                                          ║
││     │     ├───📁 keyrings                                                             ║
││     │     ├───📁 preferences.d                                                        ║
││     │     ├───📁 preferences.d.save                                                   ║
││     │     ├───📁 sources.list.d                                                       ║
││     │     └───📁 trusted.gpg.d                                                        ║
││     ├───📁 avahi                                                                      ║
││     ├───📁 bash_completion.d                                                          ║
││     ├───📁 binfmt.d                                                                   ▼
└/etc/apt/apt.conf.d─────────────────────────────────────────────────────────────────────┘
```

---

## Installation

1. Compile the application.
2. Make sure the resulting executable (`vcd` / `vcd.exe`) is available in your system **PATH**.
3. Add a small shell function (see below) to enable directory changes in your current shell session.

---

## Shell Integration

Because a child process cannot change the working directory of its parent shell,
`visual-cd` prints the selected directory to **stderr**, and a shell function performs the actual `cd`.

---

### Windows (PowerShell)

Add the following function to your PowerShell profile:

```powershell
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
        1 { } # cancelled
        default {
            if ($dir) { Write-Error $dir }
            else { Write-Error "vcd failed with exit code $status" }
        }
    }
}
```

Usage:

```powershell
ccd
```

---

### Linux (bash)

Add the following function to your `.bashrc` or `.bash_aliases`:

```bash
ccd() {
    dir=$(vcd 2>&1 >/dev/tty)
    status=$?

    case $status in
        0)
            cd "$dir"
            ;;
        1)
            ;; # cancelled
        *)
            echo "$dir" >&2
            ;;
    esac
}
```

Usage:

```bash
ccd
```

---

## Exit Codes

* `0` – directory selected successfully
* `1` – operation cancelled by the user
* other – error occurred

---

## License

MIT License




