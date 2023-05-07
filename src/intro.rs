// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

fn add(a: u32, b: i16) -> u64 {
    let sum: u64 = a + b;

    return sum;
}

fn third<A>(t: &[A; 4]) -> &A {
    todo!()
}

fn second<A: Clone>(t: &[A; 4]) -> A {
    todo!()
}

fn increment(i: &mut u64) {
    todo!()
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    todo!();
    result
}

#[cfg(test)]
mod test {

    #[test]
    fn add() {
        assert_eq!(super::add(3, 12), 15)
    }

    #[test]
    fn third() {
        assert_eq!(*super::third(&[1, 2, 3, 4]), 3)
    }

    #[test]
    fn second() {
        assert_eq!(super::second(&[1, 2, 3, 4]), 2)
    }

    #[test]
    fn increment() {
        let mut x = 4;
        super::increment(&mut x);
        assert_eq!(x, 5)
    }

    #[test]
    fn transpose() {
        let matrix = [
            [101, 102, 103], // :)
            [201, 202, 203],
            [301, 302, 303],
        ];
        let expected = [
            [101, 201, 301], // :)
            [102, 202, 302],
            [103, 203, 303],
        ];
        assert_eq!(super::transpose(matrix), expected)
    }
}
