#[derive(PartialEq, Debug)]
struct PasswordPolicy {
    byte: u8,
    positions: [usize; 2],
}
impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        self.positions
            .iter()
            .copied()
            .filter(|&index| password.as_bytes()[index] == self.byte)
            .count()
            == 1
    }
}

fn main() -> anyhow::Result<()> {
    let count = include_str!("input.txt")
        .lines()
        .map(parse_line)
        .map(Result::unwrap)
        .filter(|(policy, password)| policy.is_valid(password))
        .count();

    println!("{} passwords are valid", count);
    Ok(())
}

fn parse_line(s: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    peg::parser! {
        grammar parser() for str {
            rule number() -> usize
            = n:$(['0'..='9']+) {n.parse().unwrap()}

            rule position() -> usize
            = n:number() { n - 1}

            rule positions() -> [usize; 2]
            = first:position() "-" second:position() {[first, second]}

            rule byte() -> u8
            = letter:$(['a'..='z']) {letter.as_bytes()[0]}

            rule password() -> &'input str
            = letters:$([_]*) {letters}

            pub(crate) rule line() -> (PasswordPolicy, &'input str)
            = positions:positions() " " byte:byte() ": " password:password(){
                (PasswordPolicy {positions, byte}, password)
            }
        }
    }
    Ok(parser::line(s)?)
}
