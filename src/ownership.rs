// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

fn dup_ref(a: &str) -> (&str, &str) {
    todo!()
}

fn dup_owned(a: String) -> (String, String) {
    todo!()
}

// reuse the previous functions
fn dup_dup(a: &str) -> ((String, String), (&str, &str)) {
    todo!()
}

fn dup_usize(a: usize) -> (usize, usize) {
    todo!()
}

fn lowercase(a: String)  -> String {
    a.to_lowercase() + "!"
}

fn lower_vec(lst: &[&str]) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod test {
    #[test]
    fn dup_ref() {
        assert_eq!(super::dup_ref("lapin"), ("lapin", "lapin"))
    }

    #[test]
    fn dup_owned() {
        let s1 = String::from("lapin");
        let s2 = String::from("lapin");
        let s3 = String::from("lapin");
        assert_eq!(super::dup_owned(s1), (s2, s3))
    }

    #[test]
    fn dup_dup() {
        let s1 = String::from("lapin");
        let s2 = String::from("lapin");
        assert_eq!(super::dup_dup("lapin"), ((s1, s2), ("lapin", "lapin")))
    }

    #[test]
    fn lower_vec() {
        let lst = ["ABC", "DeF", "12"];
        let res = super::lower_vec(&lst);
        assert_eq!(res.len(), 3);
        assert_eq!(res[0], "abc!");
        assert_eq!(res[1], "def!");
        assert_eq!(res[2], "12!");
    }
}
