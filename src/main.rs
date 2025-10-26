use std::io::Write;
use chrono::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Calendar {
    last_id: u64,
    events: Vec<Event>,
}

impl Calendar {
    fn new() -> Self {
        Calendar { last_id: 0, events: Vec::new() }
    }
    
    fn load(&mut self) -> Result<()> {
        let path = ask_details("Enter path to load calendar from: ")?;
        let file =  match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                println!("Failed to read file: {}", e);
                return Ok(());
            }
        };
        let mut max_id = 0;
        for line in file.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 5 {
                let id: u64 = match parts[0].parse() {
                    Ok(num) => num,
                    Err(e) => {
                        println!("invalid event: {}", e);
                        continue;
                    },
                };
                if id > max_id {
                    max_id = id;
                }
                let event = Event {
                    id,
                    title: parts[1].to_string(),
                    date: parts[2].to_string(),
                    time: parts[3].to_string(),
                    description: parts[4].to_string(),
                };
                self.events.push(event);
            }
        }
        self.last_id = max_id;
        println!("Calendar loaded successfully.");
        Ok(())
    }

    fn save(&self) -> Result<()> {
        let path = ask_details("Enter path to save calendar: ")?;
        let mut file = std::fs::File::create(path)?;
        for event in &self.events {
            writeln!(file, "{}|{}|{}|{}|{}", event.id, event.title, event.date, event.time, event.description)?;
        }
        println!("Calendar saved successfully.");
        Ok(())
    }

    fn create_event(&mut self) -> Result<()> {
        let title = ask_details("Enter event title: ")?;
        let date = ask_details("Enter event date (YYYY-MM-DD): ")?;
        let time = ask_details("Enter event time (HH:MM): ")?;
        let description = ask_details("Enter event description: ")?;

        self.last_id += 1;

        let event = Event {
            id: self.last_id,
            title,
            date,
            time,
            description,
        };
        self.events.push(event);
        println!("Event created successfully.");
        Ok(())
    }

    fn list_events(&self) -> Result<()> {
        for event in &self.events {
            event.print();
        }
        Ok(())
    }

    fn delete_event(&mut self) -> Result<()> {
        let id: u64 = ask_u64("Enter the ID of the event to delete: ")?;
        if let Some(pos) = self.events.iter().position(|x| x.id == id) {
            self.events.remove(pos);
            println!("Event deleted successfully.");
        } else {
            println!("Event with ID {} not found.", id);
        }
        Ok(())
    }
    
    fn upcoming_events(&self) -> Result<()> {
        let now = Local::now();
        let date = now.format("%Y-%m-%d").to_string();
        let time = now.format("%H:%M").to_string();
        let mut upcoming: Vec<&Event> = self.events.iter()
            .filter(|e| e.date > date || (e.date == date && e.time >= time))
            .collect();
        upcoming.sort_by(|a, b| {
            if a.date == b.date {
                a.time.cmp(&b.time)
            } else {
                a.date.cmp(&b.date)
            }
        });
        // COULD BE DANGEROUS, IN OTHER TIME FORMATS, NEEDS REIMPLEMENTATION
        for event in upcoming {
            event.print();
        }
        Ok(())
    }

    fn update_event(&mut self) -> Result<()> {
        let id: u64 = ask_u64("Enter the ID of the event to update: ")?;
        if let Some(event) = self.events.iter_mut().find(|e| e.id == id) {
            let title = ask_details("Enter new event title (leave blank to keep current): ")?;
            if !title.is_empty() {
                event.title = title;
            }
            let date = ask_details("Enter new event date (YYYY-MM-DD) (leave blank to keep current): ")?;
            if !date.is_empty() {
                event.date = date;
            }
            let time = ask_details("Enter new event time (HH:MM) (leave blank to keep current): ")?;
            if !time.is_empty() {
                event.time = time;
            }
            let description = ask_details("Enter new event description (leave blank to keep current): ")?;
            if !description.is_empty() {
                event.description = description;
            }
            println!("Event updated successfully.");
        } else {
            println!("Event with ID {} not found.", id);
        }
        Ok(())
    }

    fn view(&self) -> Result<()> {
        let id: u64 = ask_u64("Enter the ID of the event to view: ")?;
        if let Some(event) = self.events.iter().find(|e| e.id == id) {
            event.print();
        } else {
            println!("Event with ID {} not found.", id);
        }
        Ok(())
    }

    fn search(&self) -> Result<()> {
        let query = ask_details("Enter search query: ")?;
        for event in &self.events {
            if event.title.contains(&query) || event.description.contains(&query) {
                event.print();
            }
        }
        Ok(())
    }
}

struct Event {
    id: u64,
    title: String,
    date: String,
    time: String,
    description: String,
}

impl Event {
    fn print(&self) {
        println!("==========================");
        println!("ID: {}\nTitle: {}\nDatetime: {} {}\nDescription: {}", 
                 self.id, self.title, self.date, self.time, self.description);
        println!("==========================\n");
    }
}

fn ask_details(question: &str) -> Result<String> {
    let mut input = String::new();
    println!("{}", question);
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn ask_u64(question: &str) -> Result<u64> {
    loop {
        let input = ask_details(question)?;
        match input.parse::<u64>() {
            Ok(num) => return Ok(num),
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn get_command() -> Result<String> {
        
    print!("> ");
    std::io::stdout().flush()?;
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn help() {
    println!(
        "Available commands:\n
        create   - Create a new event\n
        delete   - Delete an event\n
        list     - List all events\n
        load     - Load calendar from file\n
        save     - Save calendar to file\n
        upcoming - Show upcoming events\n
        update   - Update an event\n
        view     - View an event\n
        search   - Search for an event\n
        help     - Show this help message\n
        exit     - Exit the program\n"
    );
}

#[inline(always)]
fn handle_command(command: &str, calendar: &mut Calendar) -> Result<bool> {
    match command {
        "create" => {
            calendar.create_event()?;
            Ok(true)
        }
        "delete" => {
            calendar.delete_event()?;
            Ok(true)
        }
        "list" => {
            calendar.list_events()?;
            Ok(true)
        }
        "load" => {
            calendar.load()?;
            Ok(true)
        }
        "save" => {
            calendar.save()?;
            Ok(true)
        }
        "upcoming" => {
            calendar.upcoming_events()?;
            Ok(true)
        }
        "update" => {
            calendar.update_event()?;
            Ok(true)
        }
        "view" => {
            calendar.view()?;
            Ok(true)
        }
        "search" => {
            calendar.search()?;
            Ok(true)
        }
        "help" => {
            help();
            Ok(true)
        }
        "exit" => Ok(false),
        _ => {
            println!("Unknown command: {}", command);
            Ok(true)
        }
    }
}

fn main() -> Result<()> {
    println!("Welcome to calendar!");
    let mut calendar = Calendar::new();
    loop {   
        let command = get_command()?;
        match handle_command(&command, &mut calendar) {
            Ok(true) => continue,
            Ok(false) => break,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }
    }
    Ok(())
}
