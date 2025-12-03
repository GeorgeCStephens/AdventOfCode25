use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
        //Set total count of dupes seen
        //TODO

        //Read in the input file into a single string
        let message: String = fs::read_to_string("../input.txt")?;

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
            let top_range: i64 = both_ranges[1].parse().unwrap();

            //Loop through the range
            for check_number in bottom_range..top_range {
                //Check for dupe
                //Increment dupe seen counter
                //TODO

                println!("{}", check_number);
            }

        } 
        Ok(())
}
