// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)


#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        //类型不一致是不会相等的
        //let expect: u32 = 4;
        let expect: i32 = 4;
        let actual: i32 = 2 + 2;
        assert_eq!(expect, actual);
    }
}
