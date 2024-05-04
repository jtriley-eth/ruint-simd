# Ruint SIMD

the idea was to try and vectorize [ruint](https://github.com/recmo/uint) slices by decomposing each of the 4 limbs of 64 bits and composing them into a 4 element slice of `u64x4` values

initial benches aren't promising, my guess is it has to do with the inability to create const functions over the add functions, as computing the carry bit for each limb requires non-const functions.

cool experiment though

## pseudocode

```py
uint_arrays = [
    [a0, a1, a2, a3],
    [b0, b1, b2, b3],
    [c0, c1, c2, c3],
    [d0, d1, d2, d3],
]

simd_limbs = [
    [a0, b0, c0, d0],
    [a1, b1, c1, d1],
    [a2, b2, c2, d2],
    [a3, b3, c3, d3],
]

def carry(x, y):
    carry_mask = simd_lt(
            simd_sub([max, max, max, max], y,),
            x
        )

    return simd_select(
        carry_mask,
        [1, 1, 1, 1],
        [0, 0, 0, 0]
    )

def overflowing_add(x, y):
    return [
        simd_add(x[0], y[0]),
        simd_add(simd_add(x[1], y[1]), carry(x[0], y[0])),
        simd_add(simd_add(x[2], y[2]), carry(x[1], y[1])),
        simd_add(simd_add(x[3], y[3]), carry(x[2], y[2])),
    ]
```
