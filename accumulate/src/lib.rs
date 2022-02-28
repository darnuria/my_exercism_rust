/// What should the type of _function be?
pub fn map<T, E>(input: Vec<T>, mut function: impl FnMut(T) -> E) -> Vec<E> {
    let mut new = Vec::with_capacity(input.len());
    for i in input.into_iter() {
        new.push(function(i));
    }
    new
}
