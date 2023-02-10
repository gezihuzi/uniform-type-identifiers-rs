use uniform_type_identifiers::types::UTType;

fn main() {
    let item = UTType::new(
        "public.item",
        vec![],
        vec![],
        "Base type for the physical hierarchy.",
    );

    print!("{:?}", item);
}
