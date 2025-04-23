use std::io::Write;

enum File {
    Out(String),
    Inp(String),
    None,
}

struct Config {
    n: u64,
    file: File,
}

fn calculate_collatz(mut n: u64) -> Vec<u64> {
    let mut collatz = vec![n];

    while n != 1 {
        if n % 2 == 0 {
            n = n/2;
        } else {
            n = 3*n+1;
        }
        collatz.push(n);
    }
    collatz
}

fn create_plot(digit_counts: &[u64; 10]) {
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;
    
    let data_filename = "digit_counts.dat";
    let mut data_file = File::create(data_filename).unwrap_or_else(|err| {
        eprintln!("Błąd podczas tworzenia pliku danych: {}", err);
        std::process::exit(1);
    });
    
    writeln!(data_file, "# Digit Count").unwrap();
    writeln!(data_file, "# Digit Count").unwrap();
    for (digit, count) in digit_counts.iter().enumerate() {
        writeln!(data_file, "{} {}", digit, count).unwrap();
    }
    
    let script_filename = "plot_digits.gp";
    let mut script_file = File::create(script_filename).unwrap_or_else(|err| {
        eprintln!("Błąd podczas tworzenia skryptu GNUplot: {}", err);
        std::process::exit(1);
    });
    
    writeln!(script_file, "set terminal png size 800,600").unwrap();
    writeln!(script_file, "set output 'digit_counts.png'").unwrap();
    writeln!(script_file, "set title 'Częstotliwość występowania cyfr'").unwrap();
    writeln!(script_file, "set xlabel 'Cyfra'").unwrap();
    writeln!(script_file, "set ylabel 'Liczba wystąpień'").unwrap();
    writeln!(script_file, "set style fill solid").unwrap();
    writeln!(script_file, "set boxwidth 0.8").unwrap();
    writeln!(script_file, "set xrange [-0.5:9.5]").unwrap();
    writeln!(script_file, "set xtics 0,1,9").unwrap();
    writeln!(script_file, "set grid y").unwrap();
    writeln!(script_file, "plot 'digit_counts.dat' using 1:2 with boxes lc rgb '#4169E1' notitle, '' using 1:2:2 with labels offset 0,1 notitle").unwrap();
    
    println!("Tworzenie wykresu częstotliwości cyfr...");
    let gnuplot_result = Command::new("gnuplot")
        .arg(script_filename)
        .output();
    
    match gnuplot_result {
        Ok(_) => println!("Wykres został utworzony jako 'digit_counts.png'"),
        Err(err) => eprintln!("Błąd podczas uruchamiania gnuplot: {}", err)
    }
}

fn count_digits(filename: &str) {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};
    use std::process::Command;

    let file = File::open(filename).unwrap_or_else(|err| {
        eprintln!("Błąd podczas otwierania pliku {}: {}", filename, err);
        std::process::exit(1);
    });

    let reader = BufReader::new(file);

    let mut digit_counts = [0u64; 10];

    for line in reader.lines() {
        let line = line.unwrap_or_else(|err| {
            eprintln!("Błąd podczas czytania linii: {}", err);
            String::new()
        });

        for c in line.chars() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap() as usize;
                digit_counts[digit] += 1;
            }
        }
    }

    for (digit, count) in digit_counts.iter().enumerate() {
        if *count > 0 {
            println!("{}: {}", digit, count);
        }
    }

    let data_filename = "digit_counts.dat";
    let mut data_file = File::create(data_filename).unwrap_or_else(|err| {
        eprintln!("Błąd podczas tworzenia pliku danych: {}", err);
        std::process::exit(1);
    });

    for (digit, count) in digit_counts.iter().enumerate() {
        writeln!(data_file, "{} {}", digit, count).unwrap();
    }

    create_plot(&digit_counts);
}

fn print_collatz(cfg: Config) {
    match cfg.file{
        File::Inp(filename) => {
            count_digits(&filename);
        },
        File::Out(ref filename) => {
            let file = std::fs::File::create(filename).unwrap_or_else(|err| {
                eprintln!("Błąd podczas tworzenia pliku {}: {}", filename, err);
                std::process::exit(1);
            });

            let mut writer = std::io::BufWriter::new(file);

            for i in 1..=cfg.n {
                let collatz = calculate_collatz(i);
                let mut line = format!("{}", i); 
                for num in collatz.iter().skip(1) {
                    line.push_str(&format!(" {}", num));
                }
                line.push('\n');

                if let Err(err) = writer.write_all(line.as_bytes()) {
                    eprintln!("Błąd podczas zapisu do pliku: {}", err);
                    std::process::exit(1);
                }
            }

            if let Err(err) = writer.flush() {
                eprintln!("Błąd podczas zapisywania bufora do pliku: {}", err);
                std::process::exit(1);
            }

            println!("Zapisano sekwencje Collatza do pliku: {}", filename);
        },
        _ => {
            for i in 1..=cfg.n {
                let collatz = calculate_collatz(i);
                print!("{} ", i);
                for num in collatz.iter().skip(1) {
                    print!("{} ", num);
                }
                println!();
            }
        }
    }
}

fn main() {
    let cfg: Config = setup();
    print_collatz(cfg);
}

fn setup() -> Config {
    let mut cfg: Config = Config{ n: 10, file: File::None };
    let args: Vec<String> = std::env::args().collect();
    let mut opts =  args.iter().skip(1);
    while let Some(arg) = opts.next() {
        match arg.as_str() {
            "-n" => {
                if let Some(value) = opts.next() {
                    println!("-n value: {}", value);
                    cfg.n = value.parse::<u64>().unwrap_or_else(|_| {
                        eprintln!("Błąd! To nie jest liczba!");
                        std::process::exit(1);
                    });
                } else {
                    println!("Błąd! Nie podano argumentu po opcji -n!");
                    std::process::exit(1);
                }
            },
            "-o" => {
                if let Some(name) = opts.next() {
                    if std::path::Path::new(name).exists() {
                        eprintln!("Uwaga! Plik {} już istnieje!", name);
                        std::process::exit(1);
                    } else {
                        println!("-0 value: {}", name);
                        cfg.file = File::Out(name.to_owned());
                    }
                } else {
                    eprintln!("Błąd! nie podano nic po opcji -o!");
                    std::process::exit(1);
                }
            },
            "-i" => {
                if let Some(name) = opts.next() {
                    if !std::path::Path::new(name).exists() {
                        eprintln!("Błąd! Plik {} nie istnieje!", name);
                        std::process::exit(1);
                    } else {
                        cfg.file = File::Inp(name.to_owned());
                    }
                } else {
                    eprintln!("Błąd! nie podano nic po opcji -i!");
                    std::process::exit(1);
                }
            },
            _ => {
                println!("{}", arg);
            }
        }
    }
    cfg
}