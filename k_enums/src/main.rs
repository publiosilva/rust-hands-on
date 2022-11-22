#[allow(dead_code)]
enum WeekDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

fn is_weekend(week_day: WeekDay) -> bool {
    match week_day {
        WeekDay::Sunday | WeekDay::Saturday => true,
        _ => false,
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
    Cymk {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn print_color_name() {
    let color = Color::Cymk {
        cyan: 100,
        magenta: 50,
        yellow: 70,
        black: 200,
    };

    println!(
        "Color = {}",
        match color {
            Color::Red => "red",
            Color::Green => "green",
            Color::Blue => "blue",
            Color::RGB(0, 0, 0)
            | Color::Cymk {
                cyan: _,
                magenta: _,
                yellow: _,
                black: 255,
            } => "black",
            Color::RGB(_, _, _) => "unknown RGB",
            Color::Cymk {
                cyan: _,
                magenta: _,
                yellow: _,
                black: _,
            } => "unknown Cymk",
        }
    );
}

fn main() {
    println!("Sunday is weekend = {}", is_weekend(WeekDay::Sunday));

    print_color_name();
}
