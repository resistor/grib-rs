# Contributing to the `grib` library

All contributions including bug reports, bug fixes, documentation improvements, feature enhancements, and ideas are always welcome.

## Where to start?

Since this project is still in the early stage, there are so many things to do that I cannot write them down.  However, if multiple contributors do the same work, both of them will get unhappy.  So, it is helpful if you first make an issue before you start working so that the project can assign you.

When this project can grow to have a good community and continuous contributors, major themes will be settled and more descriptions will be written down.

## Specifying dependencies

When using other crates, we have a policy that we do not designate dependency versions too strictly.

Based on [semantic versioning](https://semver.org/), for stable crates with a major version of 1 or higher, we specify dependencies using only the major version as much as possible (i.e., `"1"` for version `"x.y.z"`). Also, for unstable crates whose major version is less than 1, we specify dependencies with minor versions as much as possible (i.e., `"0.y"` for `"0.y.z"`).

Of course, this is not the case if we need to be more specific in order to avoid bugs or API changes in a particular version.

See the [Cargo documentation](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html) for specifying dependencies.

## Before committing

### Formatting

Rust has a formal coding style as described in [Rust Style Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md).  Please use [`rustfmt`](https://github.com/rust-lang/rustfmt) for formatting code in that style.

To install `rustfmt`, you can simply run as follows:

```
rustup component add rustfmt
```

To reformat the code, you just need to run this simple command:

```
cargo fmt
```

### Testing

Testing is very important.  Please test before committing.

You can run tests using this simple command:

```
cargo test
```

Since passing tests needs successful building, the prerequisites for committing are now all fulfilled.

## Commit message

As is commonly known, Git uses the first line as a subject line.  So, please use a following style if you want to set longer commit messages:

```
<subject line>
<BLANK LINE>
<body>
```

For more details, please check past commit messages.  That is the shortest route.
