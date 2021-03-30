// Macro to simplify printing. No need for {}, {:?}
// Works similar to print() in Python.
// Uses pretty printing

#[macro_export]
macro_rules! p {
    () => {
        println!();
    };

    ($($item:expr),*) => {
        $(print!("{:#?} ",$item);)*
        p!()
    }
}
