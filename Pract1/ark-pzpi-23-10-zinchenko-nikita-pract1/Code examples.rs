/* ------- В.1 ------- */
// Стандартна структура проєкту, згенерована `cargo new`:

// Cargo.toml (Маніфест проєкту)
// [package]
// name = "my_project"
// version = "0.1.0"
// edition = "2021"
// 
// [dependencies]
// # ...

// src/main.rs (Корінь бінарного проєкту)
// fn main() {
//     println!("Hello, world!");
// }

// src/lib.rs (Корінь бібліотечного проєкту)
// pub fn my_lib_function() {
//     // ...
// }

// tests/integration_test.rs (Інтеграційні тести)
// #[test]
// fn it_works() {
//     assert_eq!(2 + 2, 4);
// }

/* ------- В.2 ------- */
// у src/lib.rs
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    mod serving { // Приватний модуль
        fn take_order() {}
    }
}

// `use` для імпорту
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // OK
    // front_of_house::serving::take_order(); // Помилка! 'serving' - приватний
}

/* ------- В.3 ------- */
// 'rustfmt' автоматично перетворить...
fn 
my_function(
    x:i32
) 
    -> i32 {
        x+1
}

// ...на ось це:
fn my_function(x: i32) -> i32 {
    x + 1
}

/* ------- В.4 ------- */
// Добре (Відповідно до конвенцій Rust)
struct UserProfile { // PascalCase
    user_name: String, // snake_case
}

fn get_user_profile(user: &UserProfile) { // snake_case
    // ...
}

// Погано (Неідіоматично)
struct user_profile { // не PascalCase
    UserName: String, // не snake_case
}

/* ------- В.5 ------- */
// Добре: Використання константи
const MAX_LOGIN_ATTEMPTS: u32 = 3; // SCREAMING_SNAKE_CASE

fn login(attempts: u32) {
    if attempts >= MAX_LOGIN_ATTEMPTS {
        // ...
    }
}

// Погано: Використання "магічного числа"
fn login_bad(attempts: u32) {
    if attempts >= 3 { // '3' - магічне число
        // ...
    }
}

/* ------- В.6 ------- */
// Погано: коментар пояснює "Що"
// Додаємо 1 до x
let x = 0;
x += 1;

// Добре: коментар пояснює "Чому"
// Ми повинні збільшити лічильник через вимоги протоколу X
let x = 0;
x += 1;

// TODO: для позначення майбутньої роботи
// FIXME: для позначення відомої помилки, що потребує виправлення

/* ------- В.7 ------- */
/// Додає два числа.
///
/// # Examples
///
/// ```
/// // Цей код буде виконано під час `cargo test`
/// assert_eq!(my_lib::add(2, 2), 4);
/// ```
///
/// # Panics
/// Ця функція ніколи не панікує.
///
/// # Safety
/// Ця функція є безпечною для виклику.
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/* ------- В.8 ------- */
// Добре: Ідіоматична обробка помилок через `Result` та оператор `?`
fn get_config_value() -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open("config.txt")?; // '?' прокидає помилку
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Погано: Використання .unwrap(), що призведе до паніки (крешу)
fn get_config_value_bad() -> String {
    let file = std::fs::File::open("config.txt").unwrap(); // Панікує, якщо файл не знайдено
    // ...
    String::new()
}

/* ------- В.9 ------- */
// Функція, яку ми тестуємо
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Модуль `tests` знаходиться в тому ж файлі
#[cfg(test)] // Цей модуль компілюється тільки при `cargo test`
mod tests {
    use super::*; // Імпортуємо `add` з батьківського модуля

    #[test] // Позначаємо функцію як unit-тест
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    #[should_panic] // Тест, який перевіряє, що функція панікує
    fn test_panic_example() {
        // panic!("Цей тест має впасти");
    }
}

/* ------- В.10 ------- */
// Використання інструментів у терміналі

// 1. Швидка перевірка синтаксису та помилок (без компіляції)
// $ cargo check

// 2. Аналіз стилю та пошук неідіоматичного коду
// $ cargo clippy

/* ------- В.11 ------- */
// ПОГАНО: панікує, якщо `value` не є числом
fn parse_bad(value: &str) -> i32 {
    // .unwrap() призведе до крешу програми, якщо parse поверне Err
    value.parse::<i32>().unwrap()
}

// ПОГАНО: імперативний C-стиль з ручним керуванням індексами
fn double_values_bad() {
    let mut v = vec![1, 2, 3];
    let mut v2 = vec![];
    for i in 0..v.len() {
        v2.push(v[i] * 2);
    }
}

/* ------- В.12 ------- */
// ДОБРЕ: повертає помилку, яку можна обробити
fn parse_good(value: &str) -> Result<i32, std::num::ParseIntError> {
    // Функція повертає `Result`, дозволяючи тому, хто її викликав,
    // вирішити, що робити з помилкою.
    value.parse::<i32>()
}

// ДОБРЕ: функціональний стиль з нульовою вартістю (0-cost abstraction)
fn double_values_good() {
    let v = vec![1, 2, 3];
    // .iter().map().collect() - ідіоматично, безпечно 
    // і компілюється в такий самий швидкий код, як і цикл for.
    let v2: Vec<i32> = v.iter().map(|x| x * 2).collect();
}