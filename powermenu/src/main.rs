use duct::cmd;

fn main() -> Result<(), std::io::Error> {
    let opts = [
        " Lock",
        "鈴 Suspend",
        " Logout",
        "勒 Reboot",
        " Shutdown",
    ];
    let output = cmd!(
        "rofi",
        "-sep",
        "|",
        "-dmenu",
        "-i",
        "-p",
        "System",
        "-width",
        "12",
        "-hide-scrollbar",
        "-line-padding",
        "4",
        "-padding",
        "20",
        "-lines",
        "5"
    )
    .stdin_bytes(opts.join("|"))
    .read()?;

    let variant = output.chars().skip(2).collect::<String>();
    match variant.as_ref() {
        "Lock" => {
            cmd!("i3lock").run()?;
        }
        "Logout" => {
            cmd!("i3-msg", "exit").run()?;
        }
        "Suspend" => {
            cmd!("i3lock").run()?;
            cmd!("systemctl", "suspend").run()?;
        }
        "Reboot" => {cmd!("systemctl", "reboot").run()?;}
        "Shutdown" => {cmd!("systemctl", "poweroff").run()?;}
        _ => {
            eprintln!("unkown variant {}", output);
            std::process::exit(1)
        }
    }

    Ok(())
}
