use crate::palais::Mingpan;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

macro_rules! render_block {
    ($mp: ident, $frame: ident, $build_block: ident, $mp_idx: expr, $target: ident, $target_idx: expr) => {
        let stars: Vec<_> = $mp.all_palais[$mp_idx]
            .stars_a
            .clone()
            .into_iter()
            .map(|s| Spans::from(Span::styled(s, Style::default().fg(Color::Red))))
            .collect();
        let paragraph_left = Paragraph::new(stars)
            .block($build_block(String::default()))
            .alignment(Alignment::Left);
        $frame.render_widget(paragraph_left, $target[$target_idx]);

        let text = vec![Spans::from(Span::styled(
            $mp.all_palais[$mp_idx].gz_name.clone(),
            Style::default().fg(Color::Blue),
        ))];
        let paragraph_right = Paragraph::new(text)
            .block($build_block($mp.all_palais[$mp_idx].name.clone()))
            .alignment(Alignment::Right);
        $frame.render_widget(paragraph_right, $target[$target_idx]);
    };
}

lazy_static! {
    static ref MINGPAN: Mutex<Mingpan> = Mutex::new(Mingpan::default());
}

pub(crate) fn display_palais(mp: Mingpan) -> Result<(), Box<dyn Error>> {
    *MINGPAN.lock().unwrap() = mp;

    // 1) prepare
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2) run
    let res = run(&mut terminal);

    // 3) quit
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

fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    // Background
    let background = Block::default()
        .title("命盘")
        .title_alignment(Alignment::Center);
    f.render_widget(background, size);

    // 3 principle vertical parts
    let vert_parts = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Top 4 blocks
    let top_horz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(vert_parts[0]);

    // Middle 3 horizontal subparts
    let mid_horz_parts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(vert_parts[1]);

    // Middle left 2 blocks
    let mid_left_parts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(mid_horz_parts[0]);

    // Middle left 2 blocks
    let mid_right_parts = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(mid_horz_parts[2]);

    // Botton 4 blocks
    let bottom_horz = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
        .split(vert_parts[2]);

    let build_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            //.style(Style::default().bg(Color::White).fg(Color::Blue))
            .title_alignment(Alignment::Center)
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let mp = MINGPAN.lock().unwrap();

    (5..9).into_iter().for_each(|idx| {
        render_block!(mp, f, build_block, idx, top_horz, idx - 5);
    });

    (11..15).rev().into_iter().for_each(|idx| {
        render_block!(mp, f, build_block, idx % 12, bottom_horz, 14 - idx);
    });

    (3..5).rev().into_iter().for_each(|idx| {
        render_block!(mp, f, build_block, idx, mid_left_parts, 4 - idx);
    });

    (9..11).into_iter().for_each(|idx| {
        render_block!(mp, f, build_block, idx, mid_right_parts, idx - 9);
    });

    // Center big block
    let paragraph = Paragraph::new(mp.info.clone())
        .block(build_block(String::default()))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, mid_horz_parts[1]);
}
