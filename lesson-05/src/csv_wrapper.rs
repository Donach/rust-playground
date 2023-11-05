pub mod csv_wrapper {
    use std::error::Error;
    use std::fmt::{Display, Formatter, Result as Result_Format};
    use std::io;

    pub struct Csv {
        pub input: Vec<String>,
        pub headers: Vec<String>,
        pub values: Vec<Vec<String>>,
        longest_elements: Vec<usize>,
    }
    impl Display for Csv {
        fn fmt(&self, f: &mut Formatter) -> Result_Format {
            // Print headers
            for (i, header) in self.headers.iter().enumerate() {
                let align = self.longest_elements[i];
                write!(f, "{:<align$}", header)?;
                if i < self.headers.len() - 1 {
                    write!(f, " | ")?;
                }
            }
            writeln!(f)?;

            // Print values
            for row in &self.values {
                for (i, value) in row.iter().enumerate() {
                    let align = self.longest_elements[i];
                    write!(f, "{:align$}", value)?;
                    if i < row.len() - 1 {
                        write!(f, " | ")?;
                    }
                }
                writeln!(f)?;
            }

            Ok(())
        }
    }

    pub fn handle_input() -> Result<Csv, Box<dyn Error>> {
        let mut input: Vec<String> = Vec::new();
        println!("Enter text line by line to transmute (or enter 'q' or empty line to quit): ");
        loop {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read input");

            line = line.trim().to_string();

            if line == "q" || line == "" {
                break;
            }

            if ! line.contains(","){
                eprintln!("The input must contain a comma");
            } else {
                input.push(line);
            }
        }

        match &input.len() == &0 {
            true => return Err("Invalid input".into()),
            false => (),
        }
        // Build the CSV now
        let mut csv: Csv = Csv {
            input: input.clone(),
            headers: Vec::new(),
            values: Vec::new(),
            longest_elements: Vec::new(),
        };
        let mut is_first_line = true;
        let mut longest_elements = Vec::with_capacity(input.capacity());
        
        for line in &input {
            if is_first_line {
                csv.headers = line.split(',').map(|s| s.to_string()).collect();
                is_first_line = false;
                for h in &csv.headers { longest_elements.push(h.len()); }
            } else {
                let mut row: Vec<String> = Vec::new();
                for (j, value) in line.split(',').enumerate() {
                    row.push(value.to_string());

                    while longest_elements.len() < j {
                        longest_elements.push(value.len());
                    }
                    if value.len() > longest_elements[j] {
                        longest_elements[j] = value.len();
                    }
                }
                csv.values.push(row);
            }
        };
        for e in longest_elements{
            csv.longest_elements.push(e);
        }
        //println!("{}", format!("{}", csv));
        Ok(csv)


        //todo!()
    }

}

