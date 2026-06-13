use core::fmt::{self, Write};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AppId {
    Launcher,
    Search,
    Editor,
    Paint,
    Calc,
    Files,
    Notes,
    Music,
    Settings,
    Fastfetch,
    Terminal,
}

pub struct AppCard {
    pub id: AppId,
    pub title: &'static str,
    pub subtitle: &'static str,
}

pub const APPS: &[AppCard] = &[
    AppCard {
        id: AppId::Search,
        title: "QSearch",
        subtitle: "A custom search surface inspired by Google",
    },
    AppCard {
        id: AppId::Editor,
        title: "QStudio",
        subtitle: "A VS Code-style editor shell for notes and code",
    },
    AppCard {
        id: AppId::Paint,
        title: "QLpaint",
        subtitle: "A tiny drawing pad with brush, fill, and canvas preview",
    },
    AppCard {
        id: AppId::Calc,
        title: "QLcalc",
        subtitle: "A pocket calculator that can evaluate simple expressions",
    },
    AppCard {
        id: AppId::Files,
        title: "QLfiles",
        subtitle: "A file browser for the OS tree and mounted resources",
    },
    AppCard {
        id: AppId::Notes,
        title: "QLnotes",
        subtitle: "A quick note pad with tags, tasks, and pinned ideas",
    },
    AppCard {
        id: AppId::Music,
        title: "QLmusic",
        subtitle: "A tiny player with a queue, playlist, and equalizer",
    },
    AppCard {
        id: AppId::Settings,
        title: "QLsettings",
        subtitle: "A system panel for theme, input, and power controls",
    },
    AppCard {
        id: AppId::Fastfetch,
        title: "QLfetch",
        subtitle: "A fast system banner with logo, specs, and uptime",
    },
    AppCard {
        id: AppId::Terminal,
        title: "QLterm",
        subtitle: "A command terminal for launching tools and scripts",
    },
];

pub fn render_launcher<W: Write>(out: &mut W) -> fmt::Result {
    writeln!(out, "QLite-OS Launcher")?;
    writeln!(out, "=================")?;
    for app in APPS {
        writeln!(out, "- {}: {}", app.title, app.subtitle)?;
    }
    Ok(())
}

pub fn render_search<W: Write>(out: &mut W) -> fmt::Result {
    writeln!(out, "QSearch")?;
    writeln!(out, "-------")?;
    writeln!(out, "[ qlite ] [search the local OS ]")?;
    writeln!(out, "Suggested results:")?;
    for (rank, result) in search_index("qlite").iter().enumerate() {
        writeln!(out, "  {}. {}", rank + 1, result)?;
    }
    Ok(())
}

pub fn render_editor<W: Write>(out: &mut W) -> fmt::Result {
    writeln!(out, "QStudio")?;
    writeln!(out, "-------")?;
    writeln!(out, "Files")?;
    writeln!(out, "  - boot/boot.S")?;
    writeln!(out, "  - c/runtime.c")?;
    writeln!(out, "  - rust/kernel.rs")?;
    writeln!(out, "Editor")?;
    writeln!(out, "  > fn main() {{")?;
    writeln!(out, "  >     println!(\"Hello from QStudio\");")?;
    writeln!(out, "  > }}")?;
    writeln!(out, "Status")?;
    writeln!(out, "  - tabs: 2")?;
    writeln!(out, "  - language: Rust")?;
    Ok(())
}

pub fn render_paint<W: Write>(out: &mut W) -> fmt::Result {
    let mut canvas = Canvas::new(24, 10);
    canvas.brush_line(2, 2, 20, 2, '#');
    canvas.brush_line(2, 3, 2, 7, '#');
    canvas.brush_line(20, 3, 20, 7, '#');
    canvas.brush_line(2, 7, 20, 7, '#');
    canvas.brush_line(5, 4, 8, 6, '*');
    canvas.brush_line(8, 6, 13, 4, '*');
    canvas.fill(11, 5, '.');

    writeln!(out, "QLpaint")?;
    writeln!(out, "-------")?;
    writeln!(out, "Tools: brush | fill | line | undo")?;
    writeln!(out, "Palette: red green blue white black")?;
    writeln!(out, "Canvas:")?;
    canvas.render(out)?;
    Ok(())
}

pub fn render_calc<W: Write>(out: &mut W) -> fmt::Result {
    let expression = "(12 + 8) / 4 + 3 * 2";
    let result = evaluate_expression(expression).unwrap_or(0);

    writeln!(out, "QLcalc")?;
    writeln!(out, "------")?;
    writeln!(out, "{} = {}", expression, result)?;
    writeln!(out, "Memory: M+ 18 | M- 4 | MC")?;
    writeln!(out, "Modes: basic | scientific | programmer")?;
    Ok(())
}

pub fn render_files<W: Write>(out: &mut W) -> fmt::Result {
    writeln!(out, "QLfiles")?;
    writeln!(out, "-------")?;
    writeln!(out, "/")?;
    writeln!(out, "  boot/")?;
    writeln!(out, "    boot.S")?;
    writeln!(out, "  c/")?;
    writeln!(out, "    runtime.c")?;
    writeln!(out, "  rust/")?;
    writeln!(out, "    kernel.rs")?;
    writeln!(out, "    apps.rs")?;
    writeln!(out, "  docs/")?;
    writeln!(out, "    roadmap.md")?;
    Ok(())
}

pub fn render_notes<W: Write>(out: &mut W) -> fmt::Result {
    let notes = [
        ("Kernel", "land keyboard input and persistent storage"),
        ("Apps", "make QLpaint save drawings to memory pages"),
        ("UI", "keep the launcher fast and keyboard-friendly"),
    ];

    writeln!(out, "QLnotes")?;
    writeln!(out, "-------")?;
    writeln!(out, "Pinned ideas")?;
    for (index, (title, text)) in notes.iter().enumerate() {
        writeln!(out, "  {}. [{}] {}", index + 1, title, text)?;
    }
    writeln!(out, "Tags: #kernel #ui #memory")?;
    writeln!(out, "Tasks: 3 open, 0 done")?;
    Ok(())
}

pub fn render_music<W: Write>(out: &mut W) -> fmt::Result {
    let playlist = [
        ("Boot Sequence", "0:32"),
        ("Low Latency", "1:08"),
        ("VGA Dreams", "2:14"),
        ("Sleep Loop", "0:45"),
    ];

    writeln!(out, "QLmusic")?;
    writeln!(out, "-------")?;
    writeln!(out, "Now playing: Boot Sequence")?;
    writeln!(out, "Queue:")?;
    for (index, (track, duration)) in playlist.iter().enumerate() {
        writeln!(out, "  {}. {} ({})", index + 1, track, duration)?;
    }
    writeln!(out, "EQ: ▁ ▂ ▅ ▇ ▆ ▃ ▂")?;
    writeln!(out, "Repeat: one | Shuffle: off")?;
    Ok(())
}

pub fn render_settings<W: Write>(out: &mut W) -> fmt::Result {
    let toggles = [
        ("Theme", "Quartz Dark"),
        ("Input", "Keyboard Only"),
        ("Audio", "Speaker 1"),
        ("Power", "Balanced"),
    ];

    writeln!(out, "QLsettings")?;
    writeln!(out, "----------")?;
    writeln!(out, "System switches")?;
    for (name, value) in toggles.iter() {
        writeln!(out, "  - {}: {}", name, value)?;
    }
    writeln!(out, "Hotkeys: Ctrl+Shift+P | Ctrl+P | Ctrl+Alt+Del")?;
    writeln!(out, "Status: ready")?;
    Ok(())
}

pub fn render_fastfetch<W: Write>(out: &mut W) -> fmt::Result {
    render_pc_stats(out)
}

pub fn render_pc_stats<W: Write>(out: &mut W) -> fmt::Result {
    let logo = [
"                                                            ", 
"                                                    ", 
"                   #################                ",
"               ########################             ",
"             ##########         ##########          ", 
"          #######                    ######         ", 
"        #######                        #####        ", 
"       ######                                       ", 
"      #####                                         ", 
"     #####                                          
     ####                                           
    ####                                            
   #####         ##                                 
   #####        ###                                 
   ####         ###                                 
  #####         ###                                 
  #####          ###            ###                 
  #####          ###           #######              
   ####          ###            ########            
   #####         ###               ########         
    #####         ###                ########       
     #####        ###                   #######     
      ######      ###                     #######   
       ########   #################         ######  
         #########    ########    #####        #    
            ###########       #########             
               #####################                
                    #############           ",       
    ];

    let fields = [
        ("OS", "QLite-OS"),
        ("Host", "QLite Board X1"),
        ("Kernel", "qlite-kernel 0.1"),
        ("Shell", "QLterm"),
        ("Uptime", "00:42"),
        ("Packages", "11 apps"),
    ];

    writeln!(out, "QLfetch")?;
    writeln!(out, "-------")?;
    for line in logo.iter() {
        writeln!(out, "{}", line)?;
    }
    writeln!(out, "")?;
    for (name, value) in fields.iter() {
        writeln!(out, "{}: {}", name, value)?;
    }
    writeln!(out, "Theme: quartz")?;
    writeln!(out, "CPU: virtual 1 core @ 3.2 GHz")?;
    writeln!(out, "Memory: 3 MB used / 16 MB total")?;
    writeln!(out, "Disk: 128 MB image / 64 MB free")?;
    writeln!(out, "GPU: VGA text mode")?;
    Ok(())
}

pub fn render_terminal<W: Write>(out: &mut W) -> fmt::Result {
    writeln!(out, "QLterm")?;
    writeln!(out, "------")?;
    let session = [
        "help",
        "stats pc",
        "ls /apps",
        "echo hello",
        "clear",
    ];

    for command in session.iter() {
        writeln!(out, "qlite@local:~$ {}", command)?;
        execute_terminal_command(command, out)?;
    }

    writeln!(out, "qlite@local:~$ ")?;
    writeln!(out, "cursor: █")?;
    Ok(())
}

pub fn render_showcase<W: Write>(out: &mut W) -> fmt::Result {
    render_launcher(out)?;
    writeln!(out, "")?;
    render_search(out)?;
    writeln!(out, "")?;
    render_editor(out)?;
    writeln!(out, "")?;
    render_paint(out)?;
    writeln!(out, "")?;
    render_calc(out)?;
    writeln!(out, "")?;
    render_files(out)?;
    writeln!(out, "")?;
    render_notes(out)?;
    writeln!(out, "")?;
    render_music(out)?;
    writeln!(out, "")?;
    render_settings(out)?;
    writeln!(out, "")?;
    render_fastfetch(out)?;
    writeln!(out, "")?;
    render_terminal(out)?;
    Ok(())
}

fn terminal_fetch() -> &'static str {
    "QLite-OS 0.1 | freestanding text desktop | ready"
}

fn execute_terminal_command<W: Write>(command: &str, out: &mut W) -> fmt::Result {
    let trimmed = trim_ascii(command);
    if trimmed.is_empty() {
        return Ok(());
    }

    let mut parts = trimmed.split_whitespace();
    let verb = parts.next().unwrap_or("");

    match verb {
        "help" => {
            writeln!(out, "commands: help stats pc ls cat echo fetch clear")?;
        }
        "stats" => match parts.next() {
            Some("pc") => {
                render_pc_stats(out)?;
            }
            Some(other) => {
                writeln!(out, "stats: unknown target '{}'", other)?;
            }
            None => {
                writeln!(out, "usage: stats pc")?;
            }
        },
        "fetch" => {
            render_pc_stats(out)?;
        }
        "ls" => {
            let path = parts.next().unwrap_or("/");
            render_ls(path, out)?;
        }
        "cat" => {
            let path = parts.next().unwrap_or("");
            render_cat(path, out)?;
        }
        "echo" => {
            let message = trimmed.strip_prefix("echo").unwrap_or("").trim_start();
            writeln!(out, "{}", message)?;
        }
        "clear" => {
            writeln!(out, "[screen cleared]")?;
        }
        _ => {
            writeln!(out, "{}: command not found", verb)?;
        }
    }

    Ok(())
}

fn render_ls<W: Write>(path: &str, out: &mut W) -> fmt::Result {
    match path {
        "/" => {
            writeln!(out, "boot  c  docs  rust")?;
        }
        "/apps" => {
            writeln!(out, "QLsearch  QLstudio  QLpaint  QLcalc  QLnotes")?;
        }
        "/rust" => {
            writeln!(out, "kernel.rs  apps.rs")?;
        }
        _ => {
            writeln!(out, "ls: cannot access '{}': no such file or directory", path)?;
        }
    }
    Ok(())
}

fn render_cat<W: Write>(path: &str, out: &mut W) -> fmt::Result {
    match path {
        "/motd" => {
            writeln!(out, "welcome to QLite-OS")?;
        }
        "/readme" => {
            writeln!(out, "freestanding rust + c + asm prototype")?;
        }
        "" => {
            writeln!(out, "cat: missing file operand")?;
        }
        _ => {
            writeln!(out, "cat: {}: not found", path)?;
        }
    }
    Ok(())
}

fn trim_ascii(text: &str) -> &str {
    let bytes = text.as_bytes();
    let mut start = 0usize;
    let mut end = bytes.len();

    while start < end && bytes[start].is_ascii_whitespace() {
        start += 1;
    }

    while end > start && bytes[end - 1].is_ascii_whitespace() {
        end -= 1;
    }

    &text[start..end]
}

fn search_index(query: &str) -> [&'static str; 3] {
    if contains_ascii_case_insensitive(query, "qlite") {
        ["QLite OS kernel docs", "QLite memory map", "QLite build status"]
    } else if contains_ascii_case_insensitive(query, "paint") {
        ["QLpaint tools", "Canvas shortcuts", "Brush settings"]
    } else {
        ["Search index", "Local packages", "System help"]
    }
}

fn contains_ascii_case_insensitive(haystack: &str, needle: &str) -> bool {
    let haystack_bytes = haystack.as_bytes();
    let needle_bytes = needle.as_bytes();

    if needle_bytes.is_empty() || needle_bytes.len() > haystack_bytes.len() {
        return false;
    }

    let limit = haystack_bytes.len() - needle_bytes.len();
    let mut start = 0usize;
    while start <= limit {
        let mut matched = true;
        let mut offset = 0usize;
        while offset < needle_bytes.len() {
            if lower_ascii(haystack_bytes[start + offset]) != lower_ascii(needle_bytes[offset]) {
                matched = false;
                break;
            }
            offset += 1;
        }
        if matched {
            return true;
        }
        start += 1;
    }

    false
}

fn lower_ascii(byte: u8) -> u8 {
    if byte >= b'A' && byte <= b'Z' {
        byte + 32
    } else {
        byte
    }
}

struct Canvas {
    width: usize,
    height: usize,
    cells: [u8; 240],
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        let mut canvas = Self {
            width,
            height,
            cells: [b' '; 240],
        };
        for cell in canvas.cells.iter_mut() {
            *cell = b' ';
        }
        canvas
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        if let Some(index) = self.index(x, y) {
            self.cells[index] = value;
        }
    }

    fn brush_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, value: u8) {
        let mut x = x0 as isize;
        let mut y = y0 as isize;
        let target_x = x1 as isize;
        let target_y = y1 as isize;
        let dx = (target_x - x).abs();
        let sx = if x < target_x { 1 } else { -1 };
        let dy = -(target_y - y).abs();
        let sy = if y < target_y { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            self.set(x as usize, y as usize, value);
            if x == target_x && y == target_y {
                break;
            }
            let twice_error = 2 * error;
            if twice_error >= dy {
                error += dy;
                x += sx;
            }
            if twice_error <= dx {
                error += dx;
                y += sy;
            }
        }
    }

    fn fill(&mut self, x: usize, y: usize, value: u8) {
        let Some(start_index) = self.index(x, y) else {
            return;
        };

        let target = self.cells[start_index];
        if target == value {
            return;
        }

        let mut stack = [(0usize, 0usize); 240];
        let mut seen = [false; 240];
        let mut stack_len = 0usize;
        stack[stack_len] = (x, y);
        seen[start_index] = true;
        stack_len = 1;

        while stack_len > 0 {
            stack_len -= 1;
            let (cx, cy) = stack[stack_len];
            let Some(index) = self.index(cx, cy) else {
                continue;
            };
            if self.cells[index] != target {
                continue;
            }
            self.cells[index] = value;

            if cx > 0 {
                let neighbor_index = self.index(cx - 1, cy).unwrap();
                if !seen[neighbor_index] {
                    stack[stack_len] = (cx - 1, cy);
                    seen[neighbor_index] = true;
                    stack_len += 1;
                }
            }
            if cx + 1 < self.width {
                let neighbor_index = self.index(cx + 1, cy).unwrap();
                if !seen[neighbor_index] {
                    stack[stack_len] = (cx + 1, cy);
                    seen[neighbor_index] = true;
                    stack_len += 1;
                }
            }
            if cy > 0 {
                let neighbor_index = self.index(cx, cy - 1).unwrap();
                if !seen[neighbor_index] {
                    stack[stack_len] = (cx, cy - 1);
                    seen[neighbor_index] = true;
                    stack_len += 1;
                }
            }
            if cy + 1 < self.height {
                let neighbor_index = self.index(cx, cy + 1).unwrap();
                if !seen[neighbor_index] {
                    stack[stack_len] = (cx, cy + 1);
                    seen[neighbor_index] = true;
                    stack_len += 1;
                }
            }
        }
    }

    fn render<W: Write>(&self, out: &mut W) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let index = row * self.width + col;
                let value = self.cells[index] as char;
                write!(out, "{}", value)?;
            }
            writeln!(out)?;
        }
        Ok(())
    }
}

fn evaluate_expression(expression: &str) -> Option<i32> {
    let mut values = [0i32; 32];
    let mut operators = [b'+'; 32];
    let mut values_len = 0usize;
    let mut operators_len = 0usize;
    let bytes = expression.as_bytes();
    let mut index = 0usize;

    while index < bytes.len() {
        match bytes[index] {
            b' ' => index += 1,
            b'0'..=b'9' => {
                let mut value = 0i32;
                while index < bytes.len() && bytes[index].is_ascii_digit() {
                    value = value * 10 + (bytes[index] - b'0') as i32;
                    index += 1;
                }
                values[values_len] = value;
                values_len += 1;
            }
            b'(' => {
                operators[operators_len] = b'(';
                operators_len += 1;
                index += 1;
            }
            b')' => {
                while operators_len > 0 && operators[operators_len - 1] != b'(' {
                    apply_top_operator(&mut values, &mut values_len, &operators, &mut operators_len)?;
                }
                operators_len = operators_len.checked_sub(1)?;
                index += 1;
            }
            op @ (b'+' | b'-' | b'*' | b'/') => {
                while operators_len > 0 && precedence(operators[operators_len - 1]) >= precedence(op) {
                    apply_top_operator(&mut values, &mut values_len, &operators, &mut operators_len)?;
                }
                operators[operators_len] = op;
                operators_len += 1;
                index += 1;
            }
            _ => return None,
        }
    }

    while operators_len > 0 {
        apply_top_operator(&mut values, &mut values_len, &operators, &mut operators_len)?;
    }

    if values_len == 1 {
        Some(values[0])
    } else {
        None
    }
}

fn apply_top_operator(
    values: &mut [i32; 32],
    values_len: &mut usize,
    operators: &[u8; 32],
    operators_len: &mut usize,
) -> Option<()> {
    if *values_len < 2 || *operators_len == 0 {
        return None;
    }
    let rhs = values[*values_len - 1];
    let lhs = values[*values_len - 2];
    let operator = operators[*operators_len - 1];
    let result = match operator {
        b'+' => lhs + rhs,
        b'-' => lhs - rhs,
        b'*' => lhs * rhs,
        b'/' if rhs != 0 => lhs / rhs,
        _ => return None,
    };
    *values_len -= 2;
    values[*values_len] = result;
    *values_len += 1;
    *operators_len -= 1;
    Some(())
}

fn precedence(operator: u8) -> i32 {
    match operator {
        b'+' | b'-' => 1,
        b'*' | b'/' => 2,
        _ => 0,
    }
}
