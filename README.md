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
| Sailfish | 49.827 us | 50.017 us | 50.196 us |
| Markup | 92.503 us | 93.098 us | 93.651 us |
| fomat | 319.38 us | 320.58 us | 321.70 us |
| Maud | 330.54 us | 333.05 us | 335.39 us |
| Horrorshow | 370.61 us | 373.35 us | 376.13 us |
| Ructe | 425.96 us | 428.45 us | 430.74 us |
| write | 460.24 us | 462.24 us | 464.08 us |
| Askama | 627.24 us | 630.07 us | 632.74 us |
| Tera | 2.2368 ms | 2.2472 ms | 2.2572 ms |
| Handlebars | 4.8676 ms | 4.9052 ms | 4.9408 ms |
| Liquid | 16.166 ms | 16.271 ms | 16.372 ms |
 
### Teams

| Library | Lower bound | Estimate | Upper bound |
| ------- | ----------: | -------: | ----------: |
| Sailfish | 77.265 ns | 77.481 ns | 77.685 ns |
| Markup | 149.19 ns | 150.76 ns | 152.30 ns |
| Maud | 317.54 ns | 319.64 ns | 321.64 ns |
| fomat | 498.64 ns | 500.62 ns | 502.41 ns |
| Horrorshow | 634.71 ns | 640.03 ns | 646.39 ns |
| write | 735.24 ns | 741.24 ns | 747.30 ns |
| Ructe | 861.10 ns | 866.71 ns | 871.81 ns |
| Askama | 884.70 ns | 888.52 ns | 892.22 ns |
| Handlebars | 5.7511 us | 5.7881 us | 5.8238 us |
| Tera | 5.8485 us | 5.8816 us | 5.9125 us |
| Liquid | 12.778 us | 12.818 us | 12.857 us |
 
## Running the benchmarks

```bash
$ cargo bench
```

Plots will be rendered if `gnuplot` is installed and will be available in the `target/criterion` folder.

##### Last ran on 21 March 2021
