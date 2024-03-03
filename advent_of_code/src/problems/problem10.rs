use std::{collections::HashMap, usize};

const DESIRED_LOW_VALUE: usize = 17;
const DESIRED_HIGH_VALUE: usize = 61;

#[derive(Debug, Clone)]
struct Bot {
    low_value: usize,
    high_value: usize,
    low_location: String,
    high_location: String,
}

impl Bot {
    fn new() -> Self {
        Self {
            low_value: usize::MAX,
            high_value: usize::MAX,
            low_location: String::new(),
            high_location: String::new(),
        }
    }

    fn send_values(
        &mut self,
        bot_val: usize,
        bots: &mut HashMap<usize, Bot>,
        outputs: &mut HashMap<usize, usize>,
    ) {
        if self.low_value == DESIRED_LOW_VALUE && self.high_value == DESIRED_HIGH_VALUE {
            println!("Bot Responsible for comparing {DESIRED_LOW_VALUE} and {DESIRED_HIGH_VALUE} is {bot_val}");
        }

        send_val_to_bot_or_output(self.low_value, &self.low_location, bots, outputs);
        send_val_to_bot_or_output(self.high_value, &self.high_location, bots, outputs);
    }

    fn is_full(&self) -> bool {
        self.low_value != usize::MAX
            && self.high_value != usize::MAX
            && self.low_location != ""
            && self.high_location != ""
    }
}

pub fn solve(data: &str) {
    let mut bots: HashMap<usize, Bot> = HashMap::new();
    let mut outputs: HashMap<usize, usize> = HashMap::new();

    for line in data.split('\n').filter(|x| x.len() > 0) {
        let components: Vec<&str> = line.split(' ').filter(|x| x.len() > 0).collect();
        match components[0] {
            "value" => handle_value_line(&components, &mut bots, &mut outputs),
            _ => setup_bot_interactions(&components, &mut bots, &mut outputs),
        }
    }

    println!(
        "Output Product: {}",
        outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap(),
    );
}

fn handle_value_line(
    components: &Vec<&str>,
    bots: &mut HashMap<usize, Bot>,
    outputs: &mut HashMap<usize, usize>,
) {
    let val_to_give = components[1].parse::<usize>().unwrap();
    let bot_val = components[5].parse::<usize>().unwrap();
    give_value_to_bot(val_to_give, bot_val, bots, outputs);
}

fn give_value_to_bot(
    val_to_give: usize,
    bot_val: usize,
    bots: &mut HashMap<usize, Bot>,
    outputs: &mut HashMap<usize, usize>,
) {
    let bot_to_update = bots.entry(bot_val).or_insert(Bot::new());
    if bot_to_update.low_value == usize::MAX || val_to_give < bot_to_update.low_value {
        bot_to_update.high_value = bot_to_update.low_value;
        bot_to_update.low_value = val_to_give;
    } else {
        bot_to_update.high_value = val_to_give;
    }

    if bot_to_update.is_full() {
        let mut cloned_bot = bot_to_update.clone();
        bot_to_update.low_value = usize::MAX;
        bot_to_update.high_value = usize::MAX;
        cloned_bot.send_values(bot_val, bots, outputs);
    }
}

fn send_val_to_bot_or_output(
    value: usize,
    location: &str,
    bots: &mut HashMap<usize, Bot>,
    outputs: &mut HashMap<usize, usize>,
) {
    if location.chars().nth(0).unwrap() == 'b' {
        give_value_to_bot(
            value,
            location[1..].parse::<usize>().unwrap(),
            bots,
            outputs,
        );
    } else {
        give_value_to_output(value, location[1..].parse::<usize>().unwrap(), outputs)
    }
}

fn give_value_to_output(
    val_to_give: usize,
    output_val: usize,
    outputs: &mut HashMap<usize, usize>,
) {
    let output_to_update = outputs.entry(output_val).or_insert(0);
    *output_to_update += val_to_give;
}

fn setup_bot_interactions(
    components: &Vec<&str>,
    bots: &mut HashMap<usize, Bot>,
    outputs: &mut HashMap<usize, usize>,
) {
    let bot_val = components[1].parse::<usize>().unwrap();
    let low_location = components[5][0..1].to_string() + components[6];
    let high_location = components[10][0..1].to_string() + components[11];
    let bot_to_update = bots.entry(bot_val).or_insert(Bot::new());
    bot_to_update.low_location = low_location;
    bot_to_update.high_location = high_location;

    if bot_to_update.is_full() {
        let mut cloned_bot = bot_to_update.clone();
        bot_to_update.low_value = usize::MAX;
        bot_to_update.high_value = usize::MAX;
        cloned_bot.send_values(bot_val, bots, outputs);
    }
}
