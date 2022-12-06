fn main() {
    let mut notes: Vec<f32> = vec![10.0, 8.8, 6.5];

    println!("Capacity = {}", notes.capacity());

    println!("{:?}", notes);

    notes.push(6.8);

    println!("Capacity = {}", notes.capacity());

    println!("{:?}", notes);

    println!("Note 1 = {}", notes[0]);

    println!(
        "Note 6 = {}",
        match notes.get(7) {
            Some(n) => *n,
            None => 0.0,
        }
    );

    for note in &notes {
        println!("Note = {}", note);
    }

    println!("{:?}", notes);

    while let Some(note) = notes.pop() {
        println!("Removed value = {}", note);
    }

    let notes_with_capacitory: Vec<f32> = Vec::with_capacity(4);

    println!("Capacity = {}", notes.capacity());

    notes.push(10.0);
    notes.push(8.0);
    notes.push(7.5);
    notes.push(9.0);

    println!("{:?}", notes_with_capacitory);
}
