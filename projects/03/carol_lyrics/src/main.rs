// Wrote a script that prints the lyrics to the Christmas carol “The Twelve Days of Christmas”

fn main() {
    let counting_numbers = [
        "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven",
        "Twelve",
    ];
    let ordinal_numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let last_line = "A partridge in a pear tree";

    print_lyrics(
        counting_numbers.len(),
        &ordinal_numbers,
        &counting_numbers,
        last_line,
    );
}

fn carol(n: usize, ordinal_day: &'static str, counting_number: &'static str) -> String {
    let lyrics = [
        format!("On the {ordinal_day} day of Christmas, my true love sent to me",),
        format!("{counting_number} turtle doves and"),
        format!("{counting_number} french hens"),
        format!("{counting_number} calling birds"),
        format!("{counting_number} golden rings"),
        format!("{counting_number} geese a-laying"),
        format!("{counting_number} swans a-swimming"),
        format!("{counting_number} maids a-milking"),
        format!("{counting_number} ladies dancing"),
        format!("{counting_number} lords a-leaping"),
        format!("{counting_number} pipers piping"),
        format!("{counting_number} drummers drumming"),
    ];
    lyrics[n].clone()
}

fn print_lyrics(
    times: usize,
    ordinal_numbers: &[&'static str],
    counting_numbers: &[&'static str],
    last_line: &'static str,
) {
    for n in 1..=times {
        // let ordinal_day_text = ordinal_numbers[(&n - 1) as usize];
        // let counting_number_text = counting_numbers[(&n - 1) as usize];

        if n == 1 {
            print!(
                "{}\n{}\n",
                carol(n - 1, ordinal_numbers[n - 1], counting_numbers[n - 1]),
                last_line
            );
            continue;
        }

        print!("\n");

        // print first line
        print!(
            "{}\n",
            carol(0, ordinal_numbers[n - 1], counting_numbers[n - 1])
        );

        // print the rest of the
        for m in (1..=n).rev() {
            if m == 1 {
                continue;
            }
            println!(
                "{}",
                carol(m - 1, ordinal_numbers[m - 1], counting_numbers[m - 1])
            );
        }

        // print last line
        print!("{last_line}");

        print!("\n");
    }
}
