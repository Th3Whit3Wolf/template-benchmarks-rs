# Rust template engine benchmarks

This repo tries to assess Rust template engine performance. Following the
download ratings from [crates.io](https://crates.io/categories/template-engine).

| Rank | Library | Description | Recent Downloads | Last Updated |
| :--- | :-----: | :---------- | ---------------: | :----------: |
| 2 | [handlebars](https://github.com/sunng87/handlebars-rust) | Handlebars templating implemented in Rust. | 457276 | 16 March 2021 |
| 3 | [tera](https://tera.netlify.com/) | Template engine based on Jinja2/Django templates | 194240 | 07 March 2021 |
| 4 | [askama](https://github.com/djc/askama) | Type-safe, compiled Jinja-like templates for Rust | 79785 | 17 November 2020 |
| 5 | [liquid](https://github.com/cobalt-org/liquid-rust) | The liquid templating language for Rust | 50388 | 27 February 2021 |
| 11 | [maud](https://maud.lambda.xyz/) | Compile-time HTML templates. | 10182 | 09 January 2021 |
| 16 | [ructe](https://github.com/kaj/ructe) | Rust Compiled Templates, efficient type-safe web page templates. | 4685 | 14 March 2021 |
| 21 | [horrorshow](https://github.com/Stebalien/horrorshow-rs) | a templating library written in rust macros | 2926 | 21 March 2020 |
| 28 | [markup](https://github.com/utkarshkukreti/markup.rs) | A blazing fast, type-safe template engine for Rust. | 2104 | 03 October 2020 |
| 33 | [sailfish](https://github.com/Kogia-sima/sailfish) | Really fast, intuitive template engine for Rust | 1161 | 23 January 2021 |
| unranked | [fomat-macros](https://github.com/krdln/fomat-macros) | Alternative syntax for print/write/format-like macros with a small templating language | 43782 | 24 February 2019 |
| unranked | [write!](https://doc.rust-lang.org/std/macro.write.html) | the std library `write!` macro | a lot | 10 February 2021 |

## Results

These results were produced by a GitHub Actions runner, which uses a Azure Standard_DS2_v2 virtual machine. 
For more information about the hardware used checkout [Microsoft Azure documentation](https://docs.microsoft.com/en-us/azure/virtual-machines/dv2-dsv2-series#dsv2-series).

As a [violin plot](https://en.wikipedia.org/wiki/Violin_plot) generated by [Criterion](https://japaric.github.io/criterion.rs/):

![Big table violin plot](big-table.svg)
![Teams violin plot](teams.svg)

Numbers, as output by Criterion:
### Big Table

| Library | Lower bound | Estimate | Upper bound |
| ------- | ----------: | -------: | ----------: |
| Sailfish | 52.638 us | 52.652 us | 52.668 us |
| Markup | 136.88 us | 137.05 us | 137.25 us |
| fomat | 367.84 us | 368.82 us | 370.11 us |
| Maud | 394.76 us | 394.95 us | 395.13 us |
| Horrorshow | 430.47 us | 430.63 us | 430.81 us |
| write | 470.28 us | 470.36 us | 470.44 us |
| Ructe | 596.67 us | 596.75 us | 596.86 us |
| Askama | 711.90 us | 712.18 us | 712.41 us |
| Tera | 2.4963 ms | 2.4972 ms | 2.4982 ms |
| Handlebars | 6.0112 ms | 6.0136 ms | 6.0165 ms |
| Liquid | 19.334 ms | 19.342 ms | 19.351 ms |
 
### Teams

| Library | Lower bound | Estimate | Upper bound |
| ------- | ----------: | -------: | ----------: |
| Sailfish | 97.989 ns | 98.343 ns | 98.956 ns |
| Markup | 211.71 ns | 211.85 ns | 211.99 ns |
| Maud | 417.72 ns | 418.11 ns | 418.50 ns |
| fomat | 552.89 ns | 552.95 ns | 553.02 ns |
| Horrorshow | 649.93 ns | 650.18 ns | 650.43 ns |
| write | 802.92 ns | 803.15 ns | 803.38 ns |
| Askama | 1.0066 us | 1.0071 us | 1.0075 us |
| Ructe | 1.0927 us | 1.0933 us | 1.0938 us |
| Tera | 6.3124 us | 6.3263 us | 6.3450 us |
| Handlebars | 6.7217 us | 6.7556 us | 6.7893 us |
| Liquid | 14.595 us | 14.622 us | 14.649 us |
 
## Running the benchmarks

```bash
$ cargo bench
```

Plots will be rendered if `gnuplot` is installed and will be available in the `target/criterion` folder.

##### Last ran on 21 March 2021
