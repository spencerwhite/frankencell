use cell::{first, cells::Cell};

fn main() {
    let (mut chars, next) = first().unwrap().token();
    let (nums, _) = next.token();

    let a = Cell::new('a');
    let b = Cell::new('b');
    let c = Cell::new('c');

    let one = Cell::new(1);
    let two = Cell::new(2);
    let three = Cell::new(3);

    println!("{}{}{}{}",
        a.borrow(&chars),
        b.borrow(&chars),
        one.borrow(&nums),
        two.borrow(&nums),
    );

    // This compiles because a token's cells can have any type:

    c.borrow(&nums);
    three.borrow(&chars);

    // However, this doesn't compile since a cell can only have one ID;

    // c.borrow(&chars);
    // three.borrow(&nums);
    

    let a_mut = a.borrow_mut(&mut chars);

    a.get();
    
    drop(a_mut);
}
