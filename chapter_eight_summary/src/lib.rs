pub mod stats {
    pub enum Median {
        None,
        One(i32),
        Two(i32,i32),
    }

    impl Median {
        pub fn print(&self) {
            match self {
                Median::None => println!("There is no median."),
                Median::One(i) => println!("Median value is: {}", i),
                Median::Two(i,j) => println!("Median values are: {} and {}", i, j),
            };
        }
    }

    pub fn calc_mean(v: &Vec<i32>) -> f64 {
        // CALCULATING THE MEAN
        let mut sum: i32 = 0;
        for arr in v.iter() {
            sum += arr;
        }
        
        // Average should be a float, not an integer
        sum as f64 / v.len() as f64
    }

    pub fn calc_median(v: &Vec<i32>) -> Median {
        let number_elements = v.len();
        let mut numbers = v.clone();
    
        // Sort the numbers from lowest to highest
        //numbers.sort_by(|a, b| a.cmp(b));
        numbers.sort();

        let index = number_elements/2;
        let mut median = Median::None;


        // If number_elements % 2 == 1 there is always 1 median, otherwise 
        // there could be 2 values
        if number_elements%2 == 1 {
            if let Some(i) = numbers.get(index) {
                median = Median::One(*i);
            }
        } else {
            let first_index = index - 1;

            // Get 'i' and 'j', the two elements for the median and check if
            // they are the same. If same, median is just that number. If
            // different, both numbers represent the median
            if let Some(i) = numbers.get(first_index) {
                if let Some(j) = numbers.get(index) {
                    median = if i == j {
                        Median::One(*i)
                    } else {
                        Median::Two(*i,*j)
                    };
                }
            }
        };

        median
    }

    pub fn calc_mode(v: &Vec<i32>) -> Vec<i32> {
        
        let mut number_frequency = std::collections::HashMap::new();
        // Loop through numbers and update frequency hash map
        // Double dereference because reference to vector which references the data
        for num in v.iter() {
            let count = number_frequency.entry(num).or_insert(0);
            *count += 1;
        }

        let mut max_freq = 0;
        let mut modes = Vec::new();
        for (num, freq) in number_frequency.iter() {
            if *freq > max_freq {
                modes.clear();
                
                modes.push(**num);
                max_freq = *freq;
            } else if *freq == max_freq {
                modes.push(**num);
            }
        }
        //modes.sort_by(|a,b| a.cmp(b));
        modes.sort();

        modes
    }
}