# btf2wit

Conversion from BTF to WIT.

```console
Usage: btf2wit [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  

Options:
  -o, --output-file <OUT_FILE>  
  -h, --help                    Print help
  -V, --version                 Print version
```

## Extract BTF info from DWARF

- Install `pahole`
```console
sudo apt install pahole
```

- Use clang or gcc to compile program with `-g` argument
```console
clang simple.c -c -o simple.bpf.o -g
```

- Use pahole to emit BTF info from DWARF and store it in the origin ELF file
```console
pahole -J simple.bpf.o
```

## Use `btf2wit` to emit WIT info from BTF

```
btf2wit simple.bpf.o -o simple.wit
```

## Use `wit-bindgen` to emit bindings for `C` and `Rust`

```console
wit-bindgen c simple.wit
wit-bindgen rust simple.wit
```

For examples, please refer to [examples/simple](examples/simple).


## License
MIT