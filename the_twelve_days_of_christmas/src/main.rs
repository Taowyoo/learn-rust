fn main() {
    let numbers: [&str; 12] = [
        "first", "second", "third", "forth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts: [&str; 12] = [
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me:",
            numbers[i]
        );
        if i == 0 {
            println!("A partridge in a pear tree");
        } else {
            for j in (0..=i).rev() {
                println!("{}", gifts[j]);
            }
        }
        println!();
    }
}
