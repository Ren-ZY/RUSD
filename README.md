# RustSoda -- Rust Stackoverflow Detector 

**Thank you for noticing our tool!**

This tool is designed to find the stackoverflow vulnerabilty in Rust crates.
It can find all **recursive functions** in one crate, including those complicate **cross-function recursive calls**. e.g.

```
fn a(){
    b();
}

fn b(){
    c();
}

fn c(){
    d();
    b();
}

fn d(){
    a();
}

There are two recursive functions in this case:
1. b()->c()->b()
2. a()->b()->c()->d()->a()
These two recursive functions share two same functions: b(), c() 
Our tool can detect these two recursive functions, including the call chain and location
```

Now we have used this tool to find all the problematic recursive functions in those stackoverflow CVEs, including:

* CVE-2018-20993:  Uncontrolled recursion leads to abort in deserialization
* CVE-2018-20994:  Stackoveflow when parsing malicious DNS packet
* CVE-2019-15542:  Uncontrolled recursion leads to abort in HTML serialization
* CVE-2019-25001:  Flaw in CBOR deserializer allows stackoveflow
* CVE-2020-35857:  Stackoverflow when resolving additional records from MX or SRV null targets
* CVE-2020-35858:  Parsing a specially crafted message can result in a stackoverflow

## Install

**1. New a file named [rust-toolchain.toml] in the rusd root directory, then write the following lines into your [rust-toolchain.toml]**.

```
[toolchain]
channel = "nightly-2020-08-24"
components = ["rustc-dev", "llvm-tools-preview", "rust-src"]
```

This file will automatically downlod the toolcahin and components to build RustSoda tool.

**2. Run "install_rustsoda.sh" in your rusd root directory.**

The **RustSoda** tool uses a shell script called `install_rustsoda.sh` to build and install.
You have to switch into the rusd root directory and run it in your bash(Linux)/zsh(MacOS). 
**Node**: You may have to use "**chomd 777 install_rustsoda.sh**" to make the script executable.

```
./install_rustsoda.sh
```

**install_rustsoda.sh** can install the binary tool into your CARGO_HOME/bin. 

In Linux, the CARGO_HOME always means ~/.cargo/.

**NOTE**: Rust has developed fast nowadays. Different channel has different rustc API and rust features. Some new version crates cannot be compiled with the old rustc compiler. We build our tool using the toolchain of nightly-2020-08-24. You can also modify the source code using the new version rustc API and change the toolchain manually if some crates cannot be compiled successfully using our tool.


## Using RustSoda

You can use this tool to detect stackoverflow vulnerability in rust crates after the installation is done. 

1. Switch into your crate directory.
2. copy the **"rust-toolchain.toml"** file into your **crate** directory.
3. Simply run **cargo rustsoda**. Then you can see the result of the detection.

**NOTE**: This tool is used on **crate** level and do not supports **workspace**.

