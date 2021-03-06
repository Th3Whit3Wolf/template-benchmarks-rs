# Rust template engine benchmarks

This repo tries to assess Rust template engine performance. Following the
download ratings from [crates.io](https://crates.io/categories/template-engine).

| Rank | Library | Description | Recent Downloads | Last Updated |
| :--- | :-----: | :---------- | ---------------: | :----------: |
| 2 | [handlebars](https://github.com/sunng87/handlebars-rust) | Handlebars templating implemented in Rust. | 511,229 | 25 May 2021 |
| 3 | [tera](https://tera.netlify.com/) | Template engine based on Jinja2/Django templates | 331,224 | 21 May 2021 |
| 4 | [askama](https://github.com/djc/askama) | Type-safe, compiled Jinja-like templates for Rust | 128,350 | 17 November 2020 |
| 5 | [liquid](https://github.com/cobalt-org/liquid-rust) | The liquid templating language for Rust | 41,713 | 27 February 2021 |
| 11 | [maud](https://maud.lambda.xyz/) | Compile-time HTML templates. | 11,676 | 09 January 2021 |
| 14 | [ructe](https://github.com/kaj/ructe) | Rust Compiled Templates, efficient type-safe web page templates. | 4,437 | 14 March 2021 |
| 24 | [sailfish](https://github.com/Kogia-sima/sailfish) | Simple, small, and extremely fast template engine for Rust | 3,402 | 06 April 2021 |
| 25 | [horrorshow](https://github.com/Stebalien/horrorshow-rs) | a templating library written in rust macros | 3,014 | 21 March 2020 |
| 28 | [markup](https://github.com/utkarshkukreti/markup.rs) | A blazing fast, type-safe template engine for Rust. | 1,973 | 24 May 2021 |
| unranked | [fomat-macros](https://github.com/krdln/fomat-macros) | Alternative syntax for print/write/format-like macros with a small templating language | 42,080 | 24 February 2019 |
| unranked | [write!](https://doc.rust-lang.org/std/macro.write.html) | the std library `write!` macro | a lot | 09 May 2021 |
## Results

These results are from 14 June 2021 (rustc 1.52.1) on a GitHub Actions runner , which uses a Azure Standard_DS2_v2 virtual machine. 
For more information about the hardware used checkout [Microsoft Azure documentation](https://docs.microsoft.com/en-us/azure/virtual-machines/dv2-dsv2-series#dsv2-series).
Your mileage may vary.

As a [violin plot](https://en.wikipedia.org/wiki/Violin_plot) generated by [Criterion](https://japaric.github.io/criterion.rs/):

![Big table violin plot](big-table.svg)
![Teams violin plot](teams.svg)

Numbers, as output by Criterion:

### Big Table

| Library | Lower bound | Estimate | Upper bound | Relative Performance |
| ------- | ----------: | -------: | ----------: | :------------------- |
| Sailfish | 43.033 us | 43.037 us | 43.040 us | 100.00% |
| Markup | 165.85 us | 165.90 us | 165.95 us | 25.94% |
| fomat | 237.91 us | 238.26 us | 238.59 us | 18.06% |
| Maud | 255.47 us | 255.89 us | 256.29 us | 16.82% |
| write | 328.40 us | 329.15 us | 329.87 us | 13.08% |
| Horrorshow | 382.26 us | 382.34 us | 382.43 us | 11.26% |
| Ructe | 430.84 us | 430.95 us | 431.06 us | 9.99% |
| Askama | 535.05 us | 535.98 us | 537.74 us | 8.03% |
| Tera | 1.7127 ms | 1.7165 ms | 1.7204 ms | 2.51% |
| Handlebars | 4.4214 ms | 4.4242 ms | 4.4269 ms | 0.97% |
| Liquid | 13.281 ms | 13.292 ms | 13.303 ms | 0.32% |
 
### Teams

| Library | Lower bound | Estimate | Upper bound | Relative Performance |
| ------- | ----------: | -------: | ----------: | :------------------- |
| Sailfish | 61.529 ns | 61.597 ns | 61.663 ns | 100.00% |
| Markup | 225.33 ns | 225.47 ns | 225.60 ns | 27.56% |
| Maud | 282.58 ns | 282.63 ns | 282.68 ns | 21.91% |
| fomat | 428.36 ns | 428.52 ns | 428.69 ns | 14.45% |
| Horrorshow | 502.17 ns | 502.84 ns | 503.49 ns | 12.33% |
| write | 649.94 ns | 650.42 ns | 651.01 ns | 9.54% |
| Askama | 675.42 ns | 676.86 ns | 678.25 ns | 9.16% |
| Ructe | 706.93 ns | 708.34 ns | 709.71 ns | 8.76% |
| Tera | 4.4679 us | 4.4721 us | 4.4765 us | 1.39% |
| Handlebars | 4.9771 us | 4.9999 us | 5.0229 us | 1.24% |
| Liquid | 9.0183 us | 9.0361 us | 9.0535 us | 0.69% |
 
## Running the benchmarks

```bash
$ cargo bench
```

Plots will be rendered if `gnuplot` is installed and will be available in the `target/criterion` folder.

