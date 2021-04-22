use std::process::*;
use std::time::*;
use std::io::Result;
use std::io::Write;
use std::thread::sleep;


const FALLBACK: &str = "fallback.mp3";

static urls: [&str; 1] = [
    "http://direct.franceinter.fr/live/franceinter-hifi.aac"
];


pub struct Player {
    // show must go on, so if there is a problem we still have 
    // a player structure without the process
    process: Option<Child>,
    current: usize,
}

impl Player {
    pub fn new() -> Self {
        Player { process: None, current: 0 }
    }

    pub fn init(&mut self) -> Result<()> {
        self.process = Some(Player::spawn()?);
        self.send_command("volume 256")?;
        self.setup()?;
        Ok(())
    }

    pub fn change_url(&mut self, next: bool) -> Result<()> {
        self.alive()?;
        if next {
            self.current += 1; 
            if self.current >= urls.len() { 
                self.current = 0;
            }
        } else {
            if self.current == 0 {
                self.current = urls.len()-1;
            } else {
                self.current -= 1;
            }
        }
        self.requeue(true)
    }

    pub fn play(&mut self) -> Result<()> {
        self.alive()?;
        self.send_command("play")
    }

    pub fn stop(&mut self) -> Result<()> {
        self.alive()?;
        self.send_command("stop")
    }

    pub fn voldown(&mut self) -> Result<()> {
        self.alive()?;
        self.send_command("voldown")
    }

    pub fn volup(&mut self) -> Result<()> {
        self.alive()?;
        self.send_command("volup")
    }

    fn alive(&mut self) -> Result<()> {
        match self.process {
            None => self.init(),
            Some(ref mut p) => match p.try_wait()? {
                Some(_) => self.respawn(),
                None => Ok(()),
            }
        }
    }

    fn respawn(&mut self) -> Result<()> {
        self.process = Some(Player::spawn()?);
        self.setup()
    }

    fn requeue(&mut self, play: bool) -> Result<()> {
        self.send_command("clear")?;
        let action = if play { "add" } else { "enqueue" };
        let cmd = format!("{} {}", action, urls[self.current]);
        self.send_command(&cmd)?;
        let cmd = format!("enqueue {}", FALLBACK);
        self.send_command(&cmd)
    }

    fn setup(&mut self) -> Result<()> {
        sleep(Duration::from_millis(300));
        self.requeue(false)?;
        self.send_command("loop on")
    }

    fn send_command(&mut self, command: &str) -> Result<()> {
        let process = match self.process {
            Some(ref mut p) => p,
            None => return Err(std::io::Error::last_os_error()),
        };
        let stdin = process.stdin.as_mut().expect("Failed to open stdin");
        let cmd = format!("{}\n", command);
        println!("Sending {}", command);
        stdin.write_all(cmd.as_bytes())?;
        Ok(())
    }

    fn spawn() -> Result<Child> {
        Command::new("vlc")
                .arg("-Irc")
                .stdin(Stdio::piped())
                .spawn()
    }

}
