// Задача

// Создать утилиту наподобие утилиты wc, которая принимает на вход файл, а на выход выдает результат — количество объектов в файле. 
// Подсчитывать следует количество символов/слов/строк в файле в зависимости от параметра:
// -с — показать количество символов в файле
// -l — вывести количество строк в файле
// -w — отобразить количество слов в файле
// По умолчанию, если не указан параметр, подсчитывается количество слов.
// Например: 
// Ввод: wc report.txt
// Вывод: 34423
// Ввод: wc -l report.txt
// Вывод: 3500



use std::fs::File;
use std::{env, io};
use std::io::{BufRead, Read};
use std::path::Path;

// Функция для подсчета символов
fn count_chars(file_name: &str){

    let mut read_file = File::open(file_name).expect("Не удалось найти файл");
    let mut str = String::new();
    read_file.read_to_string(&mut str).expect("Не удалось прочитать файл");

     let count = str.chars().count();
     println!("{}", count);
}

// Функция для подсчета слов
fn count_words(file_name: &str){

    let mut read_file = File::open(file_name).expect("Не удалось найти файл");
    let mut str = String::new();
    read_file.read_to_string(&mut str).expect("Не удалось прочитать файл");

    let count = str.split_whitespace().count();
    println!("{}", count);
}

// Функция для подсчета строк
fn count_lines(file_name: &str){

    let path = Path::new(file_name);
    let file = File::open(&path).expect("Не удалось найти файл");
    let reader = io::BufReader::new(file);

    let count = reader.lines().count();
    println!("{}", count)
}

// Функция для ввода операции и вывода результата
fn wc() {
    let args: Vec<String> = env::args().collect();

    let(command, file_name) = if args.len() == 2{
        ("-w", args[1].as_str())
    } else if args.len() == 3{
        (args[1].as_str(), args[2].as_str())
    } else {
        eprintln!("Для использования программы введите:\n\
        -с — показать количество символов в файле\n\
        -l — вывести количество строк в файле\n\
        -w — отобразить количество слов в файле");
        return;
    };

    match command {
        "-c" => count_chars(file_name),
        "-w" => count_words(file_name),
        "-l" => count_lines(file_name),
        _=> {
            eprintln!(" Введена не верная программа {}\n попробуйте еще раз", command)
        }
    }
}

fn main() {
   wc()
}
