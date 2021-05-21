pub fn bf(n: u32) -> String { format!("{} bottles", n) }
pub fn verse(n: u32) -> String {
    let (b, a, h) = match n {
        0 => ("".to_string(), "".to_string(), "".to_string()),
        1 => (
            "1 bottle".to_string(), 
            "no more bottles".to_string(), 
            "it".to_string()
        ),
        2 => (
            "2 bottles".to_string(), 
            "1 bottle".to_string(), 
            "one".to_string()
        ),
        _ => (
            bf(n),
            bf(n-1),
            "one".to_string()
        )
    };
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        _ => format!("{0} of beer on the wall, {0} of beer.\nTake {2} down and pass it around, {1} of beer on the wall.\n", b, a, h)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut ans = verse(start);
    for x in (end..start).rev() {
        ans = format!("{}\n{}", ans, verse(x));
    }
    ans
}

