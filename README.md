# Rust template engine benchmarks

This repo tries to assess Rust template engine performance. Following the
download ratings from [crates.io](https://crates.io/categories/template-engine).

| Rank | Library | Description | Recent Downloads | Last Updated |
| :--- | :-----: | :---------- | ---------------: | :----------: |
| 2 | [handlebars](https://github.com/sunng87/handlebars-rust) | Handlebars templating implemented in Rust. | 457,269 | 16 March 2021 |
| 3 | [tera](https://tera.netlify.com/) | Template engine based on Jinja2/Django templates | 196,581 | 07 March 2021 |
| 4 | [askama](https://github.com/djc/askama) | Type-safe, compiled Jinja-like templates for Rust | 80,054 | 17 November 2020 |
| 5 | [liquid](https://github.com/cobalt-org/liquid-rust) | The liquid templating language for Rust | 50,485 | 27 February 2021 |
| 11 | [maud](https://maud.lambda.xyz/) | Compile-time HTML templates. | 10,316 | 09 January 2021 |
| 15 | [ructe](https://github.com/kaj/ructe) | Rust Compiled Templates, efficient type-safe web page templates. | 4,749 | 14 March 2021 |
| 21 | [horrorshow](https://github.com/Stebalien/horrorshow-rs) | a templating library written in rust macros | 2,938 | 21 March 2020 |
| 28 | [markup](https://github.com/utkarshkukreti/markup.rs) | A blazing fast, type-safe template engine for Rust. | 2,091 | 03 October 2020 |
| 33 | [sailfish](https://github.com/Kogia-sima/sailfish) | Really fast, intuitive template engine for Rust | 1,167 | 23 January 2021 |
| unranked | [fomat-macros](https://github.com/krdln/fomat-macros) | Alternative syntax for print/write/format-like macros with a small templating language | 44,026 | 24 February 2019 |
| unranked | [write!](https://doc.rust-lang.org/std/macro.write.html) | the std library `write!` macro | a lot | 10 February 2021 |
## Results

These results are from 22 March 2021 (rustc 1.50.0) on a GitHub Actions runner , which uses a Azure Standard_DS2_v2 virtual machine. 
For more information about the hardware used checkout [Microsoft Azure documentation](https://docs.microsoft.com/en-us/azure/virtual-machines/dv2-dsv2-series#dsv2-series).
Your mileage may vary.

As a [violin plot](https://en.wikipedia.org/wiki/Violin_plot) generated by [Criterion](https://japaric.github.io/criterion.rs/):

![Big table violin plot](big-table.svg)
![Teams violin plot](teams.svg)

Numbers, as output by Criterion:

### Big Table

| Library | Lower bound | Estimate | Upper bound | Relative Performance |
| ------- | ----------: | -------: | ----------: | :------------------- |
| Sailfish | 50.715 us | 50.721 us | 50.727 us | 100.00% |
| Markup | 95.892 us | 95.926 us | 95.954 us | 52.88% |
| fomat | 326.59 us | 326.67 us | 326.74 us | 15.53% |
| Maud | 342.44 us | 342.49 us | 342.55 us | 14.81% |
| Horrorshow | 414.96 us | 417.09 us | 419.47 us | 12.16% |
| Ructe | 443.61 us | 443.73 us | 443.85 us | 11.43% |
| write | 459.49 us | 459.55 us | 459.62 us | 11.04% |
| Askama | 657.51 us | 666.09 us | 676.44 us | 7.61% |
| Tera | 2.3026 ms | 2.3033 ms | 2.3041 ms | 2.20% |
| Handlebars | 5.5156 ms | 5.5606 ms | 5.6085 ms | 0.91% |
| Liquid | 17.029 ms | 17.136 ms | 17.268 ms | 0.30% |
 
### Teams

| Library | Lower bound | Estimate | Upper bound | Relative Performance |
| ------- | ----------: | -------: | ----------: | :------------------- |
| Sailfish | 78.407 ns | 78.430 ns | 78.453 ns | 100.00% |
| Markup | 166.05 ns | 167.38 ns | 168.85 ns | 46.71% |
| Maud | 335.64 ns | 335.74 ns | 335.84 ns | 23.21% |
| fomat | 510.76 ns | 510.92 ns | 511.09 ns | 15.26% |
| Horrorshow | 602.93 ns | 603.17 ns | 603.42 ns | 12.94% |
| write | 763.87 ns | 764.02 ns | 764.20 ns | 10.21% |
| Askama | 930.19 ns | 934.66 ns | 939.66 ns | 8.34% |
| Ructe | 926.08 ns | 937.47 ns | 952.13 ns | 8.32% |
| Handlebars | 5.9423 us | 5.9741 us | 6.0066 us | 1.31% |
| Tera | 6.0573 us | 6.0644 us | 6.0722 us | 1.29% |
| Liquid | 13.062 us | 13.231 us | 13.418 us | 0.59% |
 
## Running the benchmarks

```bash
$ cargo bench
```

Plots will be rendered if `gnuplot` is installed and will be available in the `target/criterion` folder.

