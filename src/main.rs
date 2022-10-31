struct Owner {
    name: String,
}

struct Gadget<'a> {
    id: i32,
    owner: &'a Owner,
}

fn main() {
    let gadget_owner: Owner = Owner {
        name: "Gadget Man".to_string(),
    };

    let gadget1 = Gadget {
        id: 1,
        owner: &gadget_owner,
    };
    let gadget2 = Gadget {
        id: 2,
        owner: &gadget_owner,
    };

    println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);
}
