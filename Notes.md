# Notes

- std fn returns an instance of std::io::Stdin, which is a type that repres. a handle to the standard input for terminal
- reference = Gives a way to let multiple parts of the code access one piece of data without needing to copy it into memory multiple times. They are immutable by default which is why (&mut ref)
- Result type = enumerations (a type that can have a fixed set of values and those values are called the enum's variants)
  - variants of the Result type = Ok / Err
  - the purpose of these Result types is to encode error-handling info
- io::Result has the .expect method that you can call
  - if Err value -> it will display message that is passed in as an argument of expect
- a Match expression is made up of -> arms
  - an Arm consists of a pattern and the code that should be run if the value given to the beginning of the match expression fits that arm's pattern
- switching from an Expect call to a Match expression is how you generally move from crashing on an error to handeling the error
- Constants can be declared in any scope and can only be set to a constant expression, not to the result of a function call or any other value that could only be computed at runtime
- Rust naming convention for constants are all uppercase with underscores to between words
- The difference between mut and shadowing is that because a new variable is being created when using the let keyword, we can change the type of the value but reuse the same name
