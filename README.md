# ntimes

Execute a command N times

## Usage

Sends command to child process.  This starts subshells in parallel.
```
$ ntimes 100 -- curl 'https://google.com' -s -o /dev/null -w "%{starttransfer}\n"
```

To send a sync process and wait for each subsequent command to finish, pass -sync
```
$ ntimes 100 -sync -- curl 'https://google.com' -s -o /dev/null -w "%{starttransfer}\n"
```

## License

The MIT License
