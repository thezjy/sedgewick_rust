use sedgewick::symbol_table::{ordered_vec::OrderedVecST, SymbolTable};

fn test_client(st: &mut dyn SymbolTable<char, usize>) {
    dbg!(std::mem::size_of_val(&st));
    "SEACHEXAMPLE".chars().enumerate().for_each(|(i, c)| {
        st.put(c, i);
    });

    // let keys: Vec<&char> = st.keys().collect();

    // println!("{:?}", keys);
}

fn main() {
    let mut ordered_vec = OrderedVecST::new();
    let mut ordered_vec2 = OrderedVecST::new();

    let symbol_tables = vec![&mut ordered_vec, &mut ordered_vec2];

    symbol_tables.into_iter().for_each(|st| test_client(st));

    println!("{}", ordered_vec.size());
}
