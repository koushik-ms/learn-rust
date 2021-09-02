pub fn map<T, F, U>(input: Vec<T>, mut function: F) -> Vec<U>
    where F: FnMut(T) -> U
{
    let mut ans = Vec::with_capacity(input.len());
    for value in input {
        ans.push(function(value));
    }
    ans
}
