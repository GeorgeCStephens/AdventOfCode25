use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
        //Set total count of dupes seen
        let mut count: i64 = 0;

        //Read in the input file into a single string
        let mut message: String = fs::read_to_string("../input.txt")?;
        message = message.trim().to_string();

        //Split the file up by ,
        let file_splitter = message.split(",");
        let all_ranges = file_splitter.collect::<Vec<&str>>();

        //Loop through each range 
        for range in all_ranges{

            //Split the range by - 
            let range_splitter = range.split("-");
            let both_ranges = range_splitter.collect::<Vec<&str>>();
            
            //Store the bottom and top range
            let bottom_range: i64 = both_ranges[0].parse().unwrap();
            let mut top_range: i64 = both_ranges[1].parse().unwrap();
            top_range = top_range + 1;


            //Loop through the range
            for check_number in bottom_range..top_range {
                let string_number: String = check_number.to_string();
                //println!("===");
                //println!("{}", string_number);

                for length in 1..(string_number.len()/2+1) {
                    let string_to_check: String = string_number[0..length].to_string(); 
                    //println!("{}", string_to_check);

                    let mut invalid_code: String = "".to_owned();
                    let string_to_concat: String = string_to_check.to_owned();
                         
                    for i in 0..string_number.len()/string_to_check.len() {
                        invalid_code.push_str(&string_to_concat);
                    }
                    //println!("{}", invalid_code);

                    if string_number == invalid_code {
                        //println!("FOUND");
                        count = count + check_number;
                        break;
                    }
                }
            }

        }
        println!("{}", count);
        Ok(())
}
