# SQL Functions

This crate contains generated function stubs and implementations for SQL functions from various SQL dialects. The goal is to enable compute providers like [DataFusion](https://github.com/apache/arrow-datafusion) to selectively import functions from many dialects.

The goal: **functional parity between this crate and all functions in all SQL dialects**.

To contribute a function, please see our [contribution guidelines](contributing.md)

## Overview 
This crate is split up into the following compontents:
- **signatures** - contains folders for each SQL dialect. Each folder contains function signatures in `.sdf.yml` format. These signatures are then used by the generator to generate a function stub. Each function stub has a `todo!()` which must be filled in. 
- **src** - contains implementations for each function. We recommend one function per file.

## Prerequisite Setup
All functions are compiled into a single crate.

To **build** the functions crate:
``` shell
cargo build
```

To **test** the functions crate:
``` shell
cargo test
```

## Monomorphic Functions
The functions in this crate are *monomorphic* by default. This means that each function operates on a single *type* of data, and support for multiple data types results in an overloaded function. For example, a function that takes an integer and returns an integer, like int add(int a, int b), is monomorphic.

This is in contrast to polymorphic functions, which can operate on multiple types through mechanisms such as generics or type parameters. Monomorphic functions are typically simpler in terms of type checking and type inference, as there are no generic type parameters to resolve.

## Contributing
Contributions are more than welcome! Both to the list of reports, or documentation. Please carefully read [CONTRIBUTING.md](CONTRIBUTING.md)

## Using SQL Functions Crate
