fn lyrics() {
    let days: [&str; 12] = [
        "first", "second", "third", "fourth",
        "fifth", "sixth", "seventh", "eighth",
        "ninth", "tenth", "eleventh", "twelfth"
    ];
    let verses: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for x in 0..12 {
        println!("[Verse {}]\n\
        On the {} of Christmas, my true love sent to me\
        ", x+1, days[x]);

        for y in (0..x+1).rev() {
            println!("{}", verses[y]);
        }
        println!("\n");
    }
}

fn main() {
    println!("\n“The Twelve Days of Christmas” is an English \
        Christmas carol that enumerates each day by the gifts \
        given on each of the twelve days of Christmas.\n");
    lyrics();
}
