use clap::Subcommand;
use log::info;
use std::io::Write;
use std::{
    process::{Command, Stdio},
    str::FromStr,
};
use triangular_earth_calender_lib::{DateTime, Duration, Errors, Time};

#[derive(Subcommand, Debug)]
pub enum SubCmd {
    /// Prints current TEC
    Now,
    /// Starts alarm with provided Seconds
    Alarm { duration_str: String },
    /// Starts rofi to select options
    AskAlarm,
    /// Converts from Greg datetime to TEC
    Convert {
        /// Uses dataparser to convert string to Greg DateTime
        /// Converted to TEC DateTime
        date_str: String,
    },
    /// Makes Time from Hour, Minute and Second
    FromHms { hour: u32, minute: u32, second: u32 },
    /// Prints TEC Epoch
    Epoch,
}

fn trim_whitespace(s: &str) -> String {
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}

fn ask(args: &mut Vec<&str>, lines: Vec<String>) -> Result<String, Errors> {
    let input = lines.join("\n");
    let mut base_args = vec!["-dmenu"];
    base_args.append(args);
    let r = base_args.clone();
    let mut cmd = Command::new("rofi")
        .args(r)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child = match cmd.stdin.as_mut() {
        Some(s) => s,
        None => {
            return Err(Errors::Generic(
                "Could not get stdin for rofi cmd".to_string(),
            ))
        }
    };
    child.write_all(input.as_bytes())?;
    let res = cmd.wait_with_output()?;
    let out = trim_whitespace(&String::from_utf8_lossy(&res.stdout));
    Ok(out)
}

fn gen_options() -> Vec<String> {
    let mut out: Vec<String> = vec![];
    for (i, x) in (4167..=100000).step_by(4167).enumerate() {
        out.push(format!("{}/{}h", x, i + 1))
    }

    for (i, x) in (70..=8000).step_by(70).enumerate() {
        out.push(format!("{}/{}m", x, i + 1));
    }
    out
}

fn ask_alarm() -> Result<Duration, Errors> {
    let mut args = vec![];
    let lines = gen_options();
    let res = ask(&mut args, lines)?;
    let dur_num = if res.contains('/') {
        let r: Vec<&str> = res.split('/').collect();
        r[0].parse::<u64>()?
    } else {
        res.parse::<u64>()?
    };
    Ok(Duration::new(dur_num))
}

fn play_effect() {
    Command::new("mpv")
        .args([
            "--no-video",
            "--volume=66",
            "/x/shared/sound-effects/waagh.opus",
        ])
        .output()
        .expect("Error playing sound effect");
}

fn notify(mesg: &str) {
    Command::new("dunstify")
        .args([
            "--appname=TEC",
            "--replace=51239",
            "--timeout=6500",
            "Alarmed",
            mesg,
        ])
        .output()
        .expect("Could not run dunstify");
}

fn splt_secs(std_dur: Duration) -> Vec<Duration> {
    if Duration::from_secs(10000) <= std_dur {
        let r = std_dur.to_secs() / 3;
        vec![
            Duration::from_secs(r),
            Duration::from_secs(r),
            Duration::from_secs(r),
        ]
    } else if Duration::from_secs(10000) > std_dur && Duration::from_secs(4000) <= std_dur {
        let r = std_dur.to_secs() / 2;
        vec![Duration::from_secs(r), Duration::from_secs(r)]
    } else {
        vec![std_dur]
    }
}

fn alarm(dur: Duration) {
    let start_mesg = format!("Alarm {}F Started", dur);
    notify(&start_mesg);
    let splt = splt_secs(dur);
    info!("Split: {:#?}", splt);
    for s in splt {
        std::thread::sleep(s.to_std_dur());
        notify("Tic");
    }
    let mesg = format!("Alarm of {}F Finished", dur);
    notify(&mesg);
    play_effect();
}

impl SubCmd {
    pub fn run(&self) -> Result<(), Errors> {
        match self {
            SubCmd::Convert { date_str } => {
                let tec = DateTime::from_str(date_str)?;
                println!("{}", tec);
            }
            SubCmd::Now => {
                println!("{}", DateTime::now());
            }
            SubCmd::AskAlarm => {
                let dur = ask_alarm()?;
                alarm(dur);
            }
            SubCmd::Alarm { duration_str } => {
                let dur = Duration::from_str(duration_str)?;
                alarm(dur);
                println!("Need to be able to parse string to duration");
            }
            SubCmd::FromHms {
                hour,
                minute,
                second,
            } => {
                let time = Time::from_hms(*hour, *minute, *second);
                println!("Current time is :{time}");
            }
            SubCmd::Epoch => {
                println!("{}", DateTime::tec_epoch());
            }
        };

        Ok(())
    }
}
