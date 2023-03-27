use std::{
    fs::{File, OpenOptions},
    io::Write,
};

fn main() {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("my_first_calculator.py")
        .unwrap();
    file.write(b"print('Welcome to this calculator!')\n\nnum1 = int(input('Please choose your first number: '))
sign = input('What do you want to do? +, -, /, or *: ')
num2 = int(input('Please choose your second number: '))\n\n").unwrap();

    for i in 1..=50 {
        for j in 1..=50 {
            file.write(
                format!(
                    "if num1 == {} and sign == '+' and num2 == {}:
    print('{} + {} = {}')
elif num1 == {} and sign == '-' and num2 == {}:
    print('{} - {} = {}')
elif num1 == {} and sign == '/' and num2 == {}:
    print('{} / {} = {}')
elif num1 == {} and sign == '*' and num2 == {}:
    print('{} * {} = {}')
",
                    i,
                    j,
                    i,
                    j,
                    i + j,
                    i,
                    j,
                    i,
                    j,
                    i - j,
                    i,
                    j,
                    i,
                    j,
                    i / j,
                    i,
                    j,
                    i,
                    j,
                    i * j
                )
                .as_bytes(),
            )
            .unwrap();
        }
    }
}
