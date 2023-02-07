Indifferentiable Hashing
---

This is a rust implementation of indifferentiable hashing method that is proposed by Dmitrii Koshelev, originally [written in sage](https://github.com/dishport/Indifferentiable-hashing-to-ordinary-elliptic-curves-of-j-0-with-the-cost-of-one-exponentiation).

This library is a proof of concept, is not audited and should not be used in production.
This library uses Arkwork's backend and does not provide any constant-time guarantees.
__Use at your own risk__.

# Testing

`make test`

__Note__: you may need [SageMath](https://www.sagemath.org/) to validate test vectors.

# Benchmark

`cargo bench`


## Benchmark result

Collected over `1000` iterations, each with a random input.
```
indifferentiable hash/hash to group bls12-381                                                                            
                        time:   [72.090 ms 72.114 ms 72.143 ms]
indifferentiable hash/hash to group bls12-377                                                                            
                        time:   [63.077 ms 63.097 ms 63.121 ms]
indifferentiable hash/hash to curve bls12-381                                                                            
                        time:   [39.572 ms 39.583 ms 39.596 ms]
indifferentiable hash/hash to curve bls12-377                                                                            
                        time:   [31.920 ms 31.930 ms 31.942 ms]
indifferentiable hash/field exp                                                                            
                        time:   [18.762 ms 18.766 ms 18.770 ms]
SWU hash/hash to group bls12-377                                                                            
                        time:   [203.02 ms 203.24 ms 203.60 ms]
SWU hash/hash to group bls12-381                                                                            
                        time:   [176.64 ms 176.98 ms 177.39 ms]
```