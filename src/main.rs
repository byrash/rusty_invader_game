use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders_self::{
    audio::{create_audio, play, AudioActions},
    frame::{self, new_frame, Drawable},
    invaders::Invaders,
    player::Player,
    render,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = create_audio();
    // audio.play("startup");
    play(AudioActions::StartUp, &mut audio);

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a seperate thread
    let (render_tx, render_rx) = mpsc::channel();

    let render_handler = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        while let Ok(curr_frame) = render_rx.recv() {
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }

        // loop {
        //     let curr_frame = match render_rx.recv() {
        //         Ok(x) => x,
        //         Err(_) => break,
        //     };
        //     render::render(&mut stdout, &last_frame, &curr_frame, false);
        //     last_frame = curr_frame;
        // }
    });

    //Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    'gameLoop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if player.shoot() {
                            play(AudioActions::Pew, &mut audio);
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // audio.play("lose");
                        play(AudioActions::Lose, &mut audio);
                        break 'gameLoop;
                    }
                    _ => {}
                }
            }
        }

        //Updates
        player.update(delta);
        if invaders.update(delta) {
            play(AudioActions::Move, &mut audio);
        }
        if player.detect_hits(&mut invaders) {
            play(AudioActions::Explode, &mut audio);
        }
        thread::sleep(Duration::from_millis(1));

        // Win Or lose?
        if invaders.all_killed() {
            play(AudioActions::Win, &mut audio);
            break 'gameLoop;
        } else if invaders.reached_bottom() {
            play(AudioActions::Lose, &mut audio);
            break 'gameLoop;
        }

        // Draw & render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
    }

    //Cleanup
    drop(render_tx);
    render_handler.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
