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
                        time:   [66.673 ms 66.826 ms 67.030 ms]
indifferentiable hash/hash to group bls12-377                                                                            
                        time:   [63.077 ms 63.097 ms 63.121 ms]
indifferentiable hash/hash to curve bls12-381                                                                            
                        time:   [24.960 ms 24.966 ms 24.973 ms]
indifferentiable hash/hash to curve bls12-377                                                                            
                        time:   [24.062 ms 24.081 ms 24.101 ms]
indifferentiable hash/field exp                                                                            
                        time:   [18.762 ms 18.766 ms 18.770 ms]
SWU hash/hash to group bls12-377                                                                            
                        time:   [203.02 ms 203.24 ms 203.60 ms]
SWU hash/hash to group bls12-381                                                                            
                        time:   [176.64 ms 176.98 ms 177.39 ms]
```