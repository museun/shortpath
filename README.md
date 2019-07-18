# shortpath
takes a path and shortens it, similar to what fish does

also provides the current git branch, if desired

### usage:
```
Usage: shortpath [OPTIONS]

Positional arguments:
  path

Optional arguments:
  -h, --help        print help message
  -g, --git-branch  display current git branch
  -u, --unique      attempt to make the components unique
  -j, --json bool   output as json (default: true)
```

### example:
>shortpath -j false "c:/some long/path with/multiple/directories/here"
```
c:/s/p/m/d/here
```

>shortpath "c:/some long/path with/multiple/directories/here"
```json
{"branch":"","path":"c:/s/p/m/d/here"}
```

..and pretend we're in a repo with the current branch as dev
>shortpath "c:/some long/path with/multiple/directories/here" -g
```json
{"branch":"dev","path":"c:/s/p/m/d/here"}
```

>shortpath "c:/some long/path with/multiple/directories/here" -g -j false
```json
c:/s/p/m/d/here
dev
```

### this will 'uniquify' entries where there are multiple directories starting with the same character

>shortpath "c:/some long/path with/sibling/directories/here" -g -u
```json
{"branch":"master","path":"C:/s/p/sibling/d/here"}
```

>shortpath "c:/some long/path with/sibiling/directories/here" -j false -g -u
```
C:/s/p/sibling/d/here
master
```

this is useful for shortening paths for a shell prompt

e.g. for a powershell profile:
```powershell
# in $Profile.ps1
function prompt {
    write-host "[ " -nonewline -foregroundcolor blue
    $temp = ConvertFrom-Json (shortpath -g (get-location)) # shortpath.exe in your $env:PATH

    $path = $temp.path;
    $branch = $temp.branch.trim();

    write-host $path -nonewline -foregroundcolor green
    $exit = $LASTEXITCODE
    if ($null -ne $exit -and $exit -ne 0) {
        write-host "!" -nonewline -foregroundcolor yellow
        $exit = "{0:X}" -f $exit
        write-host "$exit" -nonewline -foregroundcolor red
    }

    if ($null -ne $branch -and "" -ne $branch) {
        write-host " @ " -NoNewline -foregroundcolor cyan
        write-host $branch -NoNewline -ForegroundColor Magenta
    }

    write-host " ]" -nonewline -foregroundcolor blue
    return " "
}
```
and your prompt will be:

![](./assets/screenshot.png)
