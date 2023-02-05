use cell::cells::Cell;
use cell::tokens::Token;
use cell::TokenBuilder;
use cell::first;

#[derive(Debug)]
pub struct LinkedList<'l, T, const ID: usize> {
    pub value: T,
    pub l: Option<&'l Cell<LinkedList<'l, T, ID>, ID>>,
    pub r: Option<&'l Cell<LinkedList<'l, T, ID>, ID>>,
}

fn main() {
    let ( mut token, _ ) = first().unwrap().token();

    let list1 = token.cell(LinkedList {
        value: 1,
        l: None,
        r: None,
    });

    let list2 = token.cell(LinkedList {
        value: 2,
        l: Some(&list1),
        r: None,
    });

    // This compiles:
    list1.borrow_mut(&mut token).r = Some(&list2);

    // This doesn't (even if list1 is mutable):
    // list1.get_mut().l = Some(&list2);
    
    // Accessing fields is impossible without the &token.
    let mut current = Some(list1.borrow(&token));
    while let Some(list) = current {
        println!("{}", list.value);

        current = list.r
            .map(|list_ref| list_ref.borrow(&token))
    }
}
