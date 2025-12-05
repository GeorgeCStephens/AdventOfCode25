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

                //Split the number in half
                let (left_half, right_half) = string_number.split_at(string_number.len()/2); 

                //Check for dupe
                if left_half == right_half {
                    //Increment dupe seen counter
                    println!("{}", check_number);
                    count = count + check_number;
                }
            }

        }
        println!("{}", count);
        Ok(())
}
