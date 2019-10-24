pub struct MemberRef<'a> {
    sym: SymRef<'a>,
    name: &'a str,
    descriptor: &'a str,
}