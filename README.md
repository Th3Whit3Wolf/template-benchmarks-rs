# Rust template engine benchmarks

This repo tries to assess Rust template engine performance. Following the
download ratings from [crates.io](https://crates.io/categories/template-engine).

| Rank | Library | Description | Recent Downloads | Last Updated |
| :--- | :-----: | :---------- | ---------------: | :----------: |
| 2 | [handlebars](https://github.com/sunng87/handlebars-rust) | Handlebars templating implemented in Rust. | 495,978 | 28 March 2021 |
| 3 | [tera](https://tera.netlify.com/) | Template engine based on Jinja2/Django templates | 250,910 | 12 April 2021 |
| 4 | [askama](https://github.com/djc/askama) | Type-safe, compiled Jinja-like templates for Rust | 92,720 | 17 November 2020 |
| 5 | [liquid](https://github.com/cobalt-org/liquid-rust) | The liquid templating language for Rust | 49,481 | 27 February 2021 |
| 11 | [maud](https://maud.lambda.xyz/) | Compile-time HTML templates. | 12,875 | 09 January 2021 |
| 13 | [ructe](https://github.com/kaj/ructe) | Rust Compiled Templates, efficient type-safe web page templates. | 5,206 | 14 March 2021 |
| 21 | [horrorshow](https://github.com/Stebalien/horrorshow-rs) | a templating library written in rust macros | 3,345 | 21 March 2020 |
| 29 | [markup](https://github.com/utkarshkukreti/markup.rs) | A blazing fast, type-safe template engine for Rust. | 2,159 | 28 March 2021 |
| 32 | [sailfish](https://github.com/Kogia-sima/sailfish) | Simple, small, and extremely fast template engine for Rust | 1,706 | 06 April 2021 |
| unranked | [fomat-macros](https://github.com/krdln/fomat-macros) | Alternative syntax for print/write/format-like macros with a small templating language | 44,313 | 24 February 2019 |
| unranked | [write!](https://doc.rust-lang.org/std/macro.write.html) | the std library `write!` macro | a lot | 23 March 2021 |
## Results

These results are from 19 April 2021 (rustc 1.51.0) on a GitHub Actions runner , which uses a Azure Standard_DS2_v2 virtual machine. 
For more information about the hardware used checkout [Microsoft Azure documentation](https://docs.microsoft.com/en-us/azure/virtual-machines/dv2-dsv2-series#dsv2-series).
Your mileage may vary.

As a [violin plot](https://en.wikipedia.org/wiki/Violin_plot) generated by [Criterion](https://japaric.github.io/criterion.rs/):

![Big table violin plot](big-table.svg)
![Teams violin plot](teams.svg)

Numbers, as output by Criterion:

### Big Table

| Library | Lower bound | Estimate | Upper bound | Relative Performance |
| ------- | ----------: | -------: | ----------: | :------------------- |
| Sailfish | 43.280 us | 43.707 us | 44.138 us | 100.00% |
| Markup | 80.334 us | 81.219 us | 82.148 us | 53.81% |
| fomat | 279.43 us | 282.90 us | 286.52 us | 15.45% |
| Maud | 326.67 us | 329.61 us | 332.59 us | 13.26% |
| Horrorshow | 371.41 us | 375.92 us | 381.26 us | 11.63% |
| write | 416.63 us | 420.79 us | 425.40 us | 10.39% |
| Ructe | 492.11 us | 497.01 us | 502.10 us | 8.79% |
| Askama | 586.42 us | 591.83 us | 597.70 us | 7.39% |
| Tera | 2.2100 ms | 2.2378 ms | 2.2691 ms | 1.95% |
| Handlebars | 4.8919 ms | 4.9409 ms | 4.9934 ms | 0.88% |
| Liquid | 16.672 ms | 16.855 ms | 17.045 ms | 0.26% |
 
### Teams

| Library | Lower bound | Estimate | Upper bound | Relative Performance |
| ------- | ----------: | -------: | ----------: | :------------------- |
| Sailfish | 75.674 ns | 76.484 ns | 77.397 ns | 100.00% |
| Markup | 140.50 ns | 141.73 ns | 142.97 ns | 53.52% |
| Maud | 321.71 ns | 325.65 ns | 329.99 ns | 23.31% |
| fomat | 447.83 ns | 453.11 ns | 458.75 ns | 16.78% |
| Horrorshow | 562.00 ns | 566.89 ns | 572.05 ns | 13.40% |
| write | 689.93 ns | 698.04 ns | 706.82 ns | 10.89% |
| Askama | 853.82 ns | 861.24 ns | 868.80 ns | 8.83% |
| Ructe | 929.73 ns | 940.53 ns | 951.45 ns | 8.08% |
| Tera | 5.6170 us | 5.6907 us | 5.7707 us | 1.34% |
| Handlebars | 5.6858 us | 5.7606 us | 5.8384 us | 1.32% |
| Liquid | 11.674 us | 11.851 us | 12.073 us | 0.64% |
 
## Running the benchmarks

```bash
$ cargo bench
```

Plots will be rendered if `gnuplot` is installed and will be available in the `target/criterion` folder.

