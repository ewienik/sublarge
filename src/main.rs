fn real_number(data: &[u8]) -> &[u8] {
    data.iter()
        .enumerate()
        .find(|(_, &v)| v != b'0')
        .map(|(idx, _)| &data[idx..])
        .unwrap_or(&[b'0'])
}

fn is_bigger(left: &[u8], right: &[u8]) -> Option<bool> {
    if left.len() != right.len() {
        return Some(left.len() > right.len());
    }
    left.iter()
        .zip(right.iter())
        .find(|(left, right)| left != right)
        .map(|(left, right)| left > right)
}

fn diff(big: &[u8], small: &[u8]) -> Vec<u8> {
    big.iter()
        .rev()
        .zip(small.iter().rev().chain(std::iter::repeat(&b'0')))
        .map(|(big, small)| (big - b'0', small - b'0'))
        .fold(
            (Vec::with_capacity(big.len()), 0),
            |(mut acc, mut transferred), (big, small)| {
                if big < small + transferred {
                    acc.insert(0, big + 10 - small - transferred + b'0');
                    transferred = 1;
                } else {
                    acc.insert(0, big - small - transferred + b'0');
                    transferred = 0;
                };
                (acc, transferred)
            },
        )
        .0
}

// return: txt1 - txt2
fn calculate(txt1: &str, txt2: &str) -> Option<String> {
    let data1 = real_number(txt1.as_bytes());
    let data2 = real_number(txt2.as_bytes());

    let (sign, output) = match is_bigger(data1, data2) {
        Some(true) => ("", diff(data1, data2)),
        Some(false) => ("-", diff(data2, data1)),
        None => return Some("0".into()),
    };
    Some(format!("{sign}{}", String::from_utf8(output).ok()?))
}

fn main() {
    println!("calculate: {}", calculate("0000000", "0000").unwrap());
    println!("calculate: {}", calculate("111", "111").unwrap());
    println!("calculate: {}", calculate("001", "100").unwrap());
    println!(
        "calculate: {}",
        calculate("10003453234555", "10002253134555").unwrap()
    );
    println!(
        "calculate: {}",
        calculate("10002253134555", "10003453234555").unwrap()
    );
}
