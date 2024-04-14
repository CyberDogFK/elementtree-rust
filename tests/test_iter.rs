use std::cell::RefCell;
use elementtree::{Element, QName};
use std::io::stdout;
use std::rc::Rc;

#[test]
fn iterate_mut_over_elements_with_one_nested() {
    let mut element = Element::from_reader(
        r#"
    <root>
        <element>E</element>
        <elementWithNested>
            <nestedElement>Nested value</nestedElement>
        </elementWithNested>
    </root>
    "#
            .as_bytes(),
    )
        .unwrap();
    let vec: Vec<Rc<RefCell<Element>>> = element.into_iter_elements_mut().collect();
    assert_eq!(vec.len(), 3);
    assert_eq!(vec[0].borrow().tag(), &QName::from("element"));
    assert_eq!(vec[1].borrow().tag(), &QName::from("nestedElement"));
    assert_eq!(vec[2].borrow().tag(), &QName::from("elementWithNested"));
    // dbg!(element);
}

#[test]
fn iterate_mut_over_elements_nested_array() {
    let mut element = Element::from_reader(
        r#"
    <root>
        <element>E</element>
        <elementWithNested>
            <nestedElement>Nested value</nestedElement>
            <nestedElement>Nested value 2</nestedElement>
            <nestedElement>Nested value 3</nestedElement>
            <nestedElement>Nested value 4</nestedElement>
        </elementWithNested>
    </root>
    "#
            .as_bytes(),
    )
        .unwrap();
    let vec: Vec<Rc<RefCell<Element>>> = element.into_iter_elements_mut().collect();
    assert_eq!(vec.len(), 6);
    assert_eq!(vec[0].borrow().tag(), &QName::from("element"));
    assert_eq!(vec[1].borrow().tag(), &QName::from("nestedElement"));
    assert_eq!(vec[2].borrow().tag(), &QName::from("nestedElement"));
    assert_eq!(vec[3].borrow().tag(), &QName::from("nestedElement"));
    assert_eq!(vec[4].borrow().tag(), &QName::from("nestedElement"));
    assert_eq!(vec[5].borrow().tag(), &QName::from("elementWithNested"));
    assert_eq!(vec[5].borrow().child_count(), 4);
}

#[test]
fn iterate_mut_over_elements_with_multiple_nested() {
    let mut element = Element::from_reader(
        r#"
    <root>
        <element>E</element>
        <elementWithNested>
            <nestedElement>Nested value</nestedElement>
            <twiceNestedElement>
                <nestedElement>Double nested value</nestedElement>
                <thriceNestedElement>
                    <nestedElement>Three times nested value</nestedElement>
                </thriceNestedElement>
            </twiceNestedElement>
        </elementWithNested>
    </root>
    "#
            .as_bytes(),
    )
        .unwrap();
    let vec: Vec<Rc<RefCell<Element>>> = element.into_iter_elements_mut().collect();
    assert_eq!(vec.len(), 7);
    assert_eq!(vec[0].borrow().tag(), &QName::from("element"));
    assert_eq!(vec[1].borrow().tag(), &QName::from("nestedElement"));
    assert_eq!(vec[2].borrow().tag(), &QName::from("nestedElement"));
    assert_eq!(vec[3].borrow().tag(), &QName::from("nestedElement"));
    assert_eq!(vec[4].borrow().tag(), &QName::from("thriceNestedElement"));
    assert_eq!(vec[5].borrow().tag(), &QName::from("twiceNestedElement"));
    assert_eq!(vec[6].borrow().tag(), &QName::from("elementWithNested"));
}


#[test]
fn iterate_over_elements_with_one_nested() {
    let mut element = Element::from_reader(
        r#"
    <root>
        <element>E</element>
        <elementWithNested>
            <nestedElement>Nested value</nestedElement>
        </elementWithNested>
    </root>
    "#
            .as_bytes(),
    )
        .unwrap();
    let vec: Vec<&Element> = element.iter_elements().collect();
    assert_eq!(vec.len(), 3);
    assert_eq!(vec[0].tag(), &QName::from("element"));
    assert_eq!(vec[1].tag(), &QName::from("nestedElement"));
    assert_eq!(vec[2].tag(), &QName::from("elementWithNested"));
}

#[test]
fn iterate_over_elements_nested_array() {
    let element = Element::from_reader(
        r#"
    <root>
        <element>E</element>
        <elementWithNested>
            <nestedElement>Nested value</nestedElement>
            <nestedElement>Nested value 2</nestedElement>
            <nestedElement>Nested value 3</nestedElement>
            <nestedElement>Nested value 4</nestedElement>
        </elementWithNested>
    </root>
    "#
            .as_bytes(),
    )
        .unwrap();
    let vec: Vec<&Element> = element.iter_elements().collect();
    assert_eq!(vec.len(), 6);
    assert_eq!(vec[0].tag(), &QName::from("element"));
    assert_eq!(vec[1].tag(), &QName::from("nestedElement"));
    assert_eq!(vec[2].tag(), &QName::from("nestedElement"));
    assert_eq!(vec[3].tag(), &QName::from("nestedElement"));
    assert_eq!(vec[4].tag(), &QName::from("nestedElement"));
    assert_eq!(vec[5].tag(), &QName::from("elementWithNested"));
}

#[test]
fn iterate_over_elements_with_multiple_nested() {
    let element = Element::from_reader(
        r#"
    <root>
        <element>E</element>
        <elementWithNested>
            <nestedElement>Nested value</nestedElement>
            <twiceNestedElement>
                <nestedElement>Double nested value</nestedElement>
                <thriceNestedElement>
                    <nestedElement>Three times nested value</nestedElement>
                </thriceNestedElement>
            </twiceNestedElement>
        </elementWithNested>
    </root>
    "#
            .as_bytes(),
    )
        .unwrap();
    let vec: Vec<&Element> = element.iter_elements().collect();
    assert_eq!(vec.len(), 7);
    assert_eq!(vec[0].tag(), &QName::from("element"));
    assert_eq!(vec[1].tag(), &QName::from("nestedElement"));
    assert_eq!(vec[2].tag(), &QName::from("nestedElement"));
    assert_eq!(vec[3].tag(), &QName::from("nestedElement"));
    assert_eq!(vec[4].tag(), &QName::from("thriceNestedElement"));
    assert_eq!(vec[5].tag(), &QName::from("twiceNestedElement"));
    assert_eq!(vec[6].tag(), &QName::from("elementWithNested"));
}

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
    let elements: Vec<&Element> = root.iter_elements().collect();

    assert!(!elements.is_empty(), "Elements must not be empty");
    let mut counter = 0;
    elements[3].children().into_iter().for_each(|e| {
        assert_eq!(
            e.tag(),
            &tag_name,
            "It  must find elements only with name item"
        );
        counter += 1;
    });
    assert_eq!(counter, 3, "Should found only 3 items");
}

// #[test]
// fn more_harder_test() {
//     let root = Element::from_reader(
//         r"
//     <Order>
//         <Meta>
//             <IsDropShip>false</IsDropShip>
//         </Meta>
//         <Header>
//             <OrderHeader>
//               <TradingPartnerId>620ALLPUERTORI4</TradingPartnerId>
//               <PurchaseOrderNumber>4520732404</PurchaseOrderNumber>
//               <TsetPurposeCode>00</TsetPurposeCode>
//               <PrimaryPOTypeCode>SA</PrimaryPOTypeCode>
//               <PurchaseOrderDate>2022-04-11</PurchaseOrderDate>
//               <TagWeLook>
//                 <FindingTag>someValue</FindingTag>
//               </TagWeLook>
//             </OrderHeader>
//         </Header>
//     </Order>
//     "
//         .as_bytes(),
//     )
//     .unwrap();
// 
//     let tag_name = QName::from("FindingTag");
//     let founded_elements: Vec<&Element> = root.iter_elements().collect();
// 
//     assert!(!founded_elements.is_empty(), "Elements must not be empty");
//     let mut counter = 0;
//     founded_elements.into_iter().for_each(|e| {
//         assert_eq!(e.tag(), &tag_name);
//         assert_eq!(e.text(), "someValue");
//         counter += 1;
//     });
//     assert_eq!(counter, 1, "Should found only 1 item");
// }

// #[test]
// fn on_no_found_tags_return_empty_list() {
//     let root = Element::from_reader(
//         r"
//     <Order>
//         <Meta>
//             <IsDropShip>false</IsDropShip>
//         </Meta>
//         <Header>
//             <OrderHeader>
//               <TradingPartnerId>620ALLPUERTORI4</TradingPartnerId>
//               <PurchaseOrderNumber>4520732404</PurchaseOrderNumber>
//               <TsetPurposeCode>00</TsetPurposeCode>
//               <PrimaryPOTypeCode>SA</PrimaryPOTypeCode>
//               <PurchaseOrderDate>2022-04-11</PurchaseOrderDate>
//             </OrderHeader>
//         </Header>
//     </Order>
//     "
//         .as_bytes(),
//     )
//     .unwrap();
// 
//     let tag_name = QName::from("NotExistingTag");
//     let elements: Vec<&Element> = root.iter_elements().collect();
// 
//     assert!(
//         elements.is_empty(),
//         "If no elements found must return empty vector"
//     )
// }

#[test]
fn test_on_names() {
    let mut root = Element::from_reader(
        r#"
        <root>
            <item>
                <item>
                </item>
            </item>
        </root>
        "#
        .as_bytes(),
    )
    .unwrap();
    root.children().for_each(|e| {
        let name = e.tag().name();
        let name2 = e.children().next().unwrap().tag().name();
        println!("{}", name);
    })
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
    let mut elements: Vec<Rc<RefCell<Element>>> = root.into_iter_elements_mut().collect();
    assert!(!elements.is_empty(), "Elements must not be empty");
    assert_eq!(elements[0].borrow().text(), "Item 1");
    assert_eq!(elements[1].borrow().text(), "Item 2");
    assert_eq!(elements[2].borrow().text(), "Item 3");
    // let l: Vec<&Element> = elements[0].borrow().children()
    //     .collect();
    // 
    // dbg!(l);
    // assert!(false);
    // root.to_writer(stdout()).unwrap()
}
