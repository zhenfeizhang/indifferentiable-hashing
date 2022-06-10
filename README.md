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
hash to group/sw hashing                                                                            
                        time:   [59.293 ms 59.359 ms 59.440 ms]
                      
hash to group/indifferentiable hash for bls12-381                        
                        time:   [23.641 ms 23.777 ms 23.928 ms]
                        
hash to group/indifferentiable hash for bls12-377                      
                        time:   [22.420 ms 22.447 ms 22.475 ms]
```