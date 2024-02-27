This project contains the source code of our stack overflow detection tool----RustSoda.

While running, RustSoda will first build a robust call graph on Rust MIR, then it will use Tarjan algorithm and BFS to find the entry API of SCCs in the call graph.

After that, we use modified AFL to accelrate the detection process of stack overflow bugs.

To use this tool, you need: 

1. run commend './install_rustsoda.sh'

2. then use 'cargo rustsoda' to find the the dangerous API in the sccs in the project.

3. use AFL to exercise the API which found by our tool.
