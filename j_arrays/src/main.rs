fn main() {
    let notes = [7f32; 4];

    for index in 0..notes.len() {
        println!("Note {} = {}", index + 1, notes[index]);
    }

    let notes_index: usize = 0;

    println!("Note 1 = {}", notes[notes_index]);

    let matrix = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4]
    ];

    for row in matrix {
        for column in row {
            println!("Column = {}", column);
        }
    }
}
