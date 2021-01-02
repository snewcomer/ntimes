# ntimes

Execute a command N times

Often times, I want to obtain an average/median of request timing stats for a particular endpoint.  This binary makes
it possible for you to obtain reasonable stats by executing your command ntimes.

Reasonable: You might be optimizing a specific endpoint, trimming unnecesary data.  ntimes can help you measure and obtain reasonable confidence.
Not so much: Hitting an endpiont that returns HTML.  While this may be insightful, first input delays is likely more of a concern.  Once your application/browser
receives its first bytes, there is still a long way to go before the user can start interacting with your site.

### Links

https://blog.cloudflare.com/a-question-of-timing/


## Usage

To send a sync process and wait for each subsequent command to finish.
```
$ ntimes 100 -- curl 'https://google.com' -s -o /dev/null -w "%{time_starttransfer}\n"
```

Sends command to child process.  This starts subshells in parallel.
```
$ ntimes 100 -p -- curl 'https://google.com' -s -o /dev/null -w "%{time_starttransfer}\n"
```

Lastly, to gather your metrics, you can redirect the stdout to the stdin of another command.
```
$ ntimes 100 -- curl 'https://google.com' -s -o /dev/null -w "%{time_starttransfer}\n" | percentile
```

[percentile](https://github.com/yuya-takeyama/percentile)

## License

The MIT License
