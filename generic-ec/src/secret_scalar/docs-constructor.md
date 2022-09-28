Constructs a new secret scalar

It takes original scalar by mutable reference instead of taking by value to avoid leaving
copies of scalar on stack. Scalar behind the reference will be zeroized after function
returned.
