/// FnMut provides us with functionality for all the desired cases.
pub fn map<I, O>(input: Vec<I>, mut function: impl FnMut(I) -> O) -> Vec<O> {
    // Allocate only once and early.
    let mut output = Vec::with_capacity(input.len());

    // Do the mapping.
    for x in input {
        output.push(function(x));
    }

    output
}
