use std::io::stdout;
use elementtree::{Element, QName};

#[test]
fn test_iter() {
    let root = Element::from_reader(
        r#"<?xml version="1.0"?>
    <root>
        <list>
            <item>Item 1</item>
            <item>Item 2</item>
            <item>Item 3</item>
        </list>
    </root>
    "#
        .as_bytes(),
    )
    .unwrap();
    let tag_name = QName::from("item");
    let elements: Vec<&Element> = root.iter_tag(&tag_name);

    assert!(!elements.is_empty(), "Elements must not be empty");
    let mut counter = 0;
    elements.into_iter().for_each(|e| {
        assert_eq!(
            e.tag(),
            &QName::from("item"),
            "It  must find elements only with name item"
        );
        counter += 1;
    });
    assert_eq!(counter, 3, "Should found only 3 items");
}

#[test]
fn more_harder_test() {
    let root = Element::from_reader(
        r"
    <Order>
        <Meta>
            <IsDropShip>false</IsDropShip>
        </Meta>
        <Header>
            <OrderHeader>
              <TradingPartnerId>620ALLPUERTORI4</TradingPartnerId>
              <PurchaseOrderNumber>4520732404</PurchaseOrderNumber>
              <TsetPurposeCode>00</TsetPurposeCode>
              <PrimaryPOTypeCode>SA</PrimaryPOTypeCode>
              <PurchaseOrderDate>2022-04-11</PurchaseOrderDate>
              <TagWeLook>
                <FindingTag>someValue</FindingTag>
              </TagWeLook>
            </OrderHeader>
        </Header>
    </Order>
    "
        .as_bytes(),
    )
    .unwrap();

    let tag_name = QName::from("FindingTag");
    let founded_elements = root.iter_tag(&tag_name);

    assert!(!founded_elements.is_empty(), "Elements must not be empty");
    let mut counter = 0;
    founded_elements.into_iter().for_each(|e| {
        assert_eq!(e.tag(), &tag_name);
        assert_eq!(e.text(), "someValue");
        counter += 1;
    });
    assert_eq!(counter, 1, "Should found only 1 item");
}

#[test]
fn on_no_found_tags_return_empty_list() {
    let root = Element::from_reader(
        r"
    <Order>
        <Meta>
            <IsDropShip>false</IsDropShip>
        </Meta>
        <Header>
            <OrderHeader>
              <TradingPartnerId>620ALLPUERTORI4</TradingPartnerId>
              <PurchaseOrderNumber>4520732404</PurchaseOrderNumber>
              <TsetPurposeCode>00</TsetPurposeCode>
              <PrimaryPOTypeCode>SA</PrimaryPOTypeCode>
              <PurchaseOrderDate>2022-04-11</PurchaseOrderDate>
            </OrderHeader>
        </Header>
    </Order>
    "
            .as_bytes(),
    )
        .unwrap();
    
    let tag_name = QName::from("NotExistingTag");
    let elements = root.iter_tag(&tag_name);
    
    assert!(elements.is_empty(), "If no elements found must return empty vector")
}

#[test]
fn test_on_stupidity() {
    let mut root = Element::from_reader(
        r#"
        <root>
            <item>
                <item>
                </item>
            </item>
        </root>
        "#.as_bytes()
    ).unwrap();
    root.children().for_each(|e| {
        let name = e.tag().name();
        let name2 = e.children().next().unwrap().tag().name();
        println!("{}", name);
    }
    )
}

#[test]
fn editing_tag_value() {
    let mut root = Element::from_reader(
        r#"<?xml version="1.0"?>
    <root>
        <list>
            <item>Item 1</item>
            <item>Item 2</item>
            <item>Item 3</item>
        </list>
    </root>
    "#
            .as_bytes(),
    )
        .unwrap();
    
    let tag_name = QName::from("item");
    // root.iter_tag(&tag_name).iter_mut()
    //     .map(|e| )
    // ;
    let mut elements = root.iter_tag_mut_rec(&tag_name);
    assert!(!elements.is_empty(), "Elements must not be empty");
    assert_eq!(elements[0].text(), "Item 1");
    assert_eq!(elements[1].text(), "Item 2");
    assert_eq!(elements[2].text(), "Item 3");
    
    root.to_writer(stdout()).unwrap()
}
