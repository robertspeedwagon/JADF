# JADF
Just Another Data Format.  
# Syntax  
the syntax is simple. `variable_name <- "value"`. `<-` here signifies an assignment operation, and it binds variable_name with the value "value".  
## important:  
do NOT under any circumstances omit the double quotes for the string value of a variable.  
the preferred style for writing variable names is snake_case.  
# Building  
Build with `cargo build --release` if you want optimizations, `cargo build` otherwise.
# Running  
If built with `cargo build --release`, the binary will be located under `/target/release/jadf`.
If built with cargo build the binary will be located under `/target/debug/jadf`.
In both cases, the preferred way to run the test program is with `cargo run argument`, where argument is a full or relative path to an existing file with a .jadf extension.
