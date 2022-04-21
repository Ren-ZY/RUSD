# RUSD -- Rust Stackoverflow Detector 

Thank you for noticing our tool!  : )

This tool is designed to find the stackoverflow vulnerabilty on the **crate** level.
It can find all the recursion functions in one crate, including those **cross-funtion recursive calls**.
Now we have used this tool to find all the vulnerabilities in the CVEs, including:

* CVE-2018-20993: Uncontrolled recursion leads to abort in deserialization
* CVE-2018-20994: Stackoveflow when parsing malicious DNS packet
* CVE-2019-15542: Uncontrolled recursion leads to abort in HTML serialization
* CVE-2019-25001: Flaw in CBOR deserializer allows stackoveflow
* CVE-2020-35857: Stackoverflow when resolving additional records from MX or SRV null targets
* CVE-2020-35858: Parsing a specially crafted message can result in a stackoverflow

## Install

1. First, you have to new a file named ""**rust-toolchain.toml**"" in the rusd root directory
them write the following lines into your ""**rust-toolchain.toml**""

```
[toolchain]
channel = "nightly-2021-01-03"
components = ["rustc-dev", "llvm-tools-preview", "rust-src"]
```

this file will automatically downlod the toolcahin and components to build RUSD tool.

**NOTE**: Different channel has different rustc API and rust features. We provide two versions of RUSD, separately using nightly-2020-08-24 and nightly-2021-01-03.
Besides, you can modify the source code to adapt the new version and change the toolchain manually. 

2. Secondly, The **RUSD** tool uses a shell script called `install_rusd.sh` to build and install.
You have to switch into the rusd root directory and run in your bash(Linux)/zsh(MacOS). 

```
./install_rlc.sh
```

**install_rusd.sh** can install the binary tool into your CARGO_HOME/bin. In Linux, the CARGO_HOME always means ~/.cargo/bin.


## Using RUSD

You can use this tool to detect stackoverflow vulnerability after the installation is done. 

**NOTE**: This tool is used on **crate** level and do not supports workspace.

1. First, after the intallation done, you can find **rusd** in the ~/.cargo directory.
2. Switch into your crate which you want to detect.
3. Add a toolchain file named "rust-toolchain.toml", the content in the file is the same as the file in the rusd root directory.
4. Simply run **cargo rusd**. Then you can see the result of the detection.
