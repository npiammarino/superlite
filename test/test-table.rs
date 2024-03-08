extern crate table;

use table::{TableError, EMAIL_MAX, USERNAME_MAX, *};

#[test]
fn test_table_add_length() {
    let mut table = Table::new();

    let id: u32 = 123;
    let username = std::str::from_utf8(&[b'a'; USERNAME_MAX]).unwrap();
    let email = std::str::from_utf8(&[b'a'; EMAIL_MAX]).unwrap();

    let username_bad = std::str::from_utf8(&[b'a'; USERNAME_MAX + 1]).unwrap();
    let email_bad = std::str::from_utf8(&[b'a'; EMAIL_MAX + 1]).unwrap();

    let r1 = Row::build(id, String::from(username), String::from(email));
    let r2 = Row::build(id, String::from(username_bad), String::from(email));
    let r3 = Row::build(id, String::from(username), String::from(email_bad));

    assert_eq!(table.push_row(r1).expect("test ok"), ());
    assert_eq!(
        format!("{:?}", table.push_row(r2).expect_err("test err")),
        "ByteOverflow"
    );
    assert_eq!(
        format!("{:?}", table.push_row(r3).expect_err("test err")),
        "ByteOverflow"
    );
}

#[test]
fn test_table_get_row() {
    let mut table = Table::new();

    let id: u32 = 123;
    let username = std::str::from_utf8(&[b'a'; USERNAME_MAX]).unwrap();
    let email = std::str::from_utf8(&[b'a'; EMAIL_MAX]).unwrap();

    let _ = table.push_row(Row::build(id, String::from(username), String::from(email)));

    let row = table.get_row(1).expect("test ok");
    assert_eq!(row.id, id);
    assert_eq!(row.username, username);
    assert_eq!(row.email, email);

    assert_eq!(
        format!("{:?}", table.get_row(2).expect_err("test err")),
        "TableIndexError"
    )
}
