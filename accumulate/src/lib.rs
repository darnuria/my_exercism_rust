// Question for future me or anybody: How to avoid allocation?
pub fn map<T, E>(input: Vec<T>, mut function: impl FnMut(T) -> E) -> Vec<E> {
    let mut new = Vec::with_capacity(input.len());
    for i in input.into_iter() {
        new.push(function(i));
    }
    new
}
