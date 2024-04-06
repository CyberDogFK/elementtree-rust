use elementtree::{Element, QName};

#[test]
fn test_iter() {
    let root = Element::from_reader(
        r#"<?xml version="1.0"?>
    <root>
        <list>
            <item>Item 1</item>Tail 1
            <item>Item 2</item>Tail 2
            <item>Item 3</item>Tail 3
        </list>
    </root>
    "#
        .as_bytes(),
    )
    .unwrap();
    let tag_name = QName::from("item");
    let elements: Vec<&Element> = root.iter_tag(&tag_name);

    assert!(!elements.is_empty(), "Elements must not be empty");
    elements.into_iter().for_each(|e| {
        assert_eq!(
            e.tag(),
            &QName::from("item"),
            "It  must find elements only with name item"
        )
    });
}
