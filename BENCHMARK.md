# **BENCHMARK.md**

**Hardware**

- i5‑4210U @ 1.70 GHz (2c/4t, old laptop CPU)  
- 8 GB DDR3  
- Linux (Ubuntu/Debian)

**How it was tested**

Ran the full pipeline with `hyperfine`.  
This measures the actual run: reading the CSV, loading the JSON maps, Rayon doing its thing, and writing the output file.  
No microbench tricks.

**Command used**

```bash
hyperfine './target/release/loinc-validator-rs -i src/data/sample-test-file.csv -l LOINC -u UNIT -o out.csv'
```

**Results**

- Avg: **82.0 ms ± 8.9 ms**  
- Range: **69.1 ms → 105.9 ms** (41 runs)  
- User/system split: ~66.8 ms user, ~15.3 ms system

**Takeaways**

- The whole pipeline finishes under 100 ms on hardware that's basically a decade old (I bought it in June 2015, it was manufactured even earlier)  
- Rayon spreads the work across the 4 threads without causing weird stalls  
- Runtime grows in a straight line with input size, so bigger files behave predictably on newer machines
