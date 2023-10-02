use uiautomation::Result;
use uiautomation::UIAutomation;
use uiautomation::UIElement;
use uiautomation::UITreeWalker;
use uiautomation::processes::Process;
use std::fmt;
use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
    let automation = UIAutomation::new().unwrap();
    let walker = automation.get_control_view_walker().unwrap();
    let root = automation.get_root_element().unwrap();
    // Process::create("notepad").unwrap();
    Process::create("C:\\program files\\audacity\\audacity.exe").unwrap();
    sleep(Duration::new(5, 0)); 
    let matcher = automation.create_matcher().from(root.clone()).timeout(10000).name("Audacity");

    if let Ok(audacity) = matcher.find_first() { 
        sleep(Duration::new(1, 0)); 
        print_element(&walker, &audacity, 0).unwrap();
        sleep(Duration::new(1, 0)); 
     }

    let my_ele = find_element(&automation, &root, "OK", Duration::new(10, 0)).unwrap();
    my_ele.click().unwrap();

    let my_ele = find_element(&automation, &root, "Audio Setup", Duration::new(10, 0)).unwrap();
    my_ele.click().unwrap();
 
}

#[derive(Debug)]
struct FindElementError {
    message: String,
}

impl fmt::Display for FindElementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FindElementError {}

fn find_element(
    automation: &UIAutomation,
    parent: &UIElement,
    name: &str,
    timeout: Duration,
) -> std::result::Result<UIElement, FindElementError> {
    let start_time = Instant::now();
    
    loop {
        let elapsed = start_time.elapsed();
        if elapsed > timeout {
            return Err(FindElementError { message: "Timeout reached while waiting for element".to_string() });
        }

        let matcher = automation.create_matcher().from(parent.clone()).name(name);
        
        if let Ok(element) = matcher.find_first() {
            println!("Found element: {} - {}", element.get_name().unwrap(), element.get_classname().unwrap());
            return Ok(element);
        }

        // Sleep for a brief moment before checking again to avoid busy-waiting
        sleep(Duration::from_millis(100));
    }
}

fn print_element(walker: &UITreeWalker, element: &UIElement, level: usize) -> Result<()> {
    for _ in 0..level {
        print!(" ")
    }
    println!("{} - {}", element.get_classname()?, element.get_name()?);

    if let Ok(child) = walker.get_first_child(&element) {
        print_element(walker, &child, level + 1)?;

        let mut next = child;
        while let Ok(sibling) = walker.get_next_sibling(&next) {
            print_element(walker, &sibling, level + 1)?;

            next = sibling;
        }
    }
    
    Ok(())
}