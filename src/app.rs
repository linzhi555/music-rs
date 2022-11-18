use crate::mp3::Player;
use std::fs;
use std::thread;
use std::time;
use std::time::Duration;
//fn main() {
//    println!("this is a music app");
//    println!("v1.0");
//    play("/home/lin/Music/SerumProtein - タンポポ.mp3");
//}

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame, Terminal,
};

#[derive(Debug)]
enum PlayState {
    Idle,
    Running,
    Pause,
}

struct App {
    playstate: PlayState,
    state: TableState,
    items: Vec<Vec<String>>,
    player: Player,
    messages: String,
}

impl App {
    fn new(musics: Vec<Vec<String>>) -> App {
        App {
            playstate: PlayState::Idle,
            state: TableState::default(),
            items: musics,
            player: Player::new(),
            messages: String::from("welcome"),
        }
    }
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn cur_music(&mut self) -> String {
        let i = match self.state.selected() {
            Some(i) => i,
            None => 0,
        };
        return self.items.get(i).unwrap().get(0).unwrap().to_string();
    }
    pub fn start_play_cur_music(&mut self) {
        self.playstate = PlayState::Running;
        let music = self.cur_music();
        self.player.play(&music);
        self.messages = String::from("playnewsome");
    }
    pub fn toggle_pause(&mut self) {
        self.playstate = PlayState::Pause;
        self.player.toggle_pause();
    }
}

fn find_musics(path: &str) -> Vec<Vec<String>> {
    let paths = fs::read_dir(path).unwrap();
    let mut musics: Vec<Vec<String>> = vec![];
    for path in paths {
        let file_name = path.unwrap().path().to_str().unwrap().to_string();
        if file_name.ends_with(".mp3") {
            musics.push(vec![file_name]);
        }
    }
    musics
}

pub fn run() -> Result<(), Box<dyn Error>> {
    //let musics = find_musics("./");
    //println!("{:?}",musics);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new(find_musics("./"));
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        app.messages = format!("{:?}", time::SystemTime::now());

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Down => app.next(),
                    KeyCode::Up => app.previous(),
                    KeyCode::Enter => app.start_play_cur_music(),
                    KeyCode::Char(' ') => app.toggle_pause(),
                    _ => {}
                }
            }
        } else {
            app.messages = format!("{:?}", time::SystemTime::now());
            //if app.player.empty() {
            //    app.messages = format!("the play is empty {:?} {:?}",app.playstate , time::SystemTime::now())
            //}else{
            //    app.messages = format!("the play is not empty {:?} {:?}",app.playstate , time::SystemTime::now())
            //}

            match app.playstate {
                PlayState::Running => {
                    if app.player.empty() {
                        //if a{
                        app.next();
                        app.start_play_cur_music();
                        app.messages = format!("{:?} {:?} ,new song ", app.playstate, app.messages)
                    } else {
                        app.messages =
                            format!("{:?} {:?} ,no new song ", app.playstate, app.messages)
                    }
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(80), Constraint::Length(1)].as_ref())
        .margin(5)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["you musics"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = app.items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(c.as_str()));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);
    f.render_stateful_widget(t, rects[0], &mut app.state);

    let (msg, style) = (vec![Span::raw(app.messages.as_str())], Style::default());

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, rects[1]);

    //    let messages: Vec<ListItem> = app
    //        .messages
    //        .iter()
    //        .enumerate()
    //        .map(|(i, m)| {
    //            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
    //            ListItem::new(content)
    //        })
    //        .collect();
    //    let messages =
    //        List::new(messages).block(Block::default().borders(Borders::ALL).title("Mess"));
    //    f.render_widget(messages, rects[0]);
}
