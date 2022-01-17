# What

This project embeds 3 web servers (java, nodejs and rust) to do a basic performance comparison on hello world.

# How-to

Clone the repo

## Java (java 8 is required) 
```
cd javap
./mvnw spring-boot:run
```

## Node (nodejs is required)
```
cd nodep
node index.js
```

## Rust (rust is required)
```
cd rustp
cargo run
```

# Tests

I am using wrk

https://github.com/wg/wrk

Just clone the repo and cd and ./make


# Results

Tested on my mac (2.2 GHz 6-Core Intel Core i7 / 16 GB 2400 MHz DDR4)

```
wrk -t8 -c256 -d30s http://127.0.0.1:8080
```


## Java (Spring boot)
```
Running 30s test @ http://127.0.0.1:8080
  8 threads and 256 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.81ms    9.28ms 325.66ms   97.66%
    Req/Sec     6.08k     0.93k   10.14k    87.82%
  1455330 requests in 30.10s, 173.75MB read
  Socket errors: connect 0, read 237, write 0, timeout 0
Requests/sec:  48349.85
Transfer/sec:      5.77MB
```

## Nodejs
```
Running 30s test @ http://127.0.0.1:8080
  8 threads and 256 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     7.10ms    1.25ms  64.07ms   97.43%
    Req/Sec     4.55k   362.90     6.10k    95.14%
  1088683 requests in 30.10s, 167.16MB read
  Socket errors: connect 0, read 320, write 0, timeout 0
Requests/sec:  36164.65
Transfer/sec:      5.55MB
```

## Rust (actix)
```
Running 30s test @ http://127.0.0.1:8080
  8 threads and 256 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     3.33ms    1.84ms  43.88ms   88.63%
    Req/Sec    10.09k     0.90k   25.60k    77.90%
  2411596 requests in 30.10s, 296.68MB read
  Socket errors: connect 0, read 104, write 0, timeout 0
Requests/sec:  80117.43
Transfer/sec:      9.86MB
```
