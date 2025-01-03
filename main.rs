use cursive::view::{Nameable, Resizable, Scrollable};
use cursive::views::{Dialog, TextView, EditView, ScrollView};
use cursive::{Cursive, CursiveExt};
use std::process::Command;
use std::thread;
use sysinfo::*;


fn fetch_task_list() -> String {
    let output = Command::new("cmd")
        .args(&["/c", "tasklist"])
        .output()
        .expect("Failed to execute `tasklist` command");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn main() {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_memory();
    let mem = sys.total_memory();
    let used = sys.used_memory();
    let cores = sys.cpus().len();
    
    let mut siv = Cursive::default();

    sys.refresh_all();

println!("=> system:");
// RAM and swap information:
println!("total memory: {} bytes", sys.total_memory());
println!("used memory : {} bytes", sys.used_memory());
println!("total swap  : {} bytes", sys.total_swap());
println!("used swap   : {} bytes", sys.used_swap());

// Display system information:
println!("System name:             {:?}", System::name());
println!("System kernel version:   {:?}", System::kernel_version());
println!("System OS version:       {:?}", System::os_version());
println!("System host name:        {:?}", System::host_name());
thread::sleep_ms(2000);

    // Set console title
    Command::new("cmd")
        .args(&["/c", "title Task Manager. You can resize to expand the task list"])
        .output()
        .expect("Error executing command");

    // Fetch the initial task list
    let task_list = fetch_task_list();

    // Add a scrollable view for the task list
    siv.add_layer(
        Dialog::around(
            ScrollView::new(TextView::new(task_list.clone()).with_name("main_view"))
                .scrollable()
                .fixed_size((80, 20)),
        )
        .title("Windows Task Manager CLI")
        .button("Refresh", move |s| {
            let refreshed_output = fetch_task_list();

            // Update the main view with the refreshed task list
            if let Some(_) = s.call_on_name("main_view", |v: &mut TextView| {
                v.set_content(refreshed_output);
            }) {
                // Successfully updated the content
            } else {
                s.add_layer(Dialog::info("Failed to find the main view!"));
            }
        })
        .button("End Task", |s| {
            s.add_layer(
                Dialog::around(EditView::new()
                    .on_submit(|s, text| {
                        s.pop_layer();
                        let task_name = text.trim();

                        if task_name.eq_ignore_ascii_case("all") {
                            s.add_layer(Dialog::info("Ending all tasks is disabled for safety."));
                        } else if task_name.is_empty() {
                            s.add_layer(Dialog::info("Please enter a valid task name or PID."));
                        } else {
                            let result = Command::new("cmd")
                                .args(&["/c", &format!("taskkill /F /IM {}", task_name)])
                                .output();

                            match result {
                                Ok(_) => s.add_layer(Dialog::info(format!("Task '{}' ended.", task_name))),
                                Err(_) => s.add_layer(Dialog::info(format!("Failed to end task '{}'.", task_name))),
                            }
                        }
                    })
                    .with_name("task_input")
                    .fixed_width(40))
                .title("Enter Task Name or PID to End"),
            );
        })
        .button("Quit", |s| s.quit())
        .button("About", |s| {
            s.add_layer(
                Dialog::around(TextView::new(
                    "Task Manager CLI is a simple command-line interface for Windows' Task List.\n\
                     It allows you to view and manipulate your running processes.\n\
                     \n\
                     Features:\n\
                     - Refresh the task list\n\
                     - End tasks by PID or name\n\
                     - Resize the window to expand the task list\n\
                     - Quit the application\n\
                     - Resources"
                ))
                .title("About")
                .button("Close", |s| { s.pop_layer(); }),
            );
        })
        .button("Resources", move |s| {
            s.add_layer(
                Dialog::around(TextView::new(
                    format!("Total memory: {} bytes\nUsed memory: {} bytes\nTotal CPU cores: {}", 
                            mem, used, cores),
                ))
                .title("Resources")
                .button("Close", |s| { s.pop_layer(); }),
            );
        })
    );

    siv.run();
}
