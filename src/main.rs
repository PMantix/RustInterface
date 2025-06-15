mod state;
mod llm;

use std::io::{self, Write};
use std::path::Path;
use dotenv::dotenv;
use state::GameState;

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let state_path = Path::new("state.json");
    let mut game_state = GameState::load(state_path)?;

    loop {
        print!("\n> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            game_state.save(state_path)?;
            println!("Exiting game. State saved.");
            break;
        }

        // Build prompt using current game state
        let prompt = format!(
            "Player: {} ({})\nReputation: {}\nShip: {}\nLocation: {}\nHull: {}\nShields: {}\nMission log: {:?}\nCommand: {}\nRespond narratively as the ship's computer.",
            game_state.player_name,
            game_state.rank,
            game_state.reputation,
            game_state.ship_name,
            game_state.location,
            game_state.hull_status,
            game_state.shield_status,
            game_state.mission_log.last().unwrap_or(&String::new()),
            input
        );

        match llm::send_prompt(&prompt) {
            Ok(reply) => {
                println!("{}", reply.trim());
                game_state.mission_log.push(format!("{} -> {}", input, reply.trim()));
                // TODO: Parse JSON deltas from the LLM to modify game state.
            }
            Err(e) => eprintln!("Error communicating with LLM: {e}"),
        }

        game_state.save(state_path)?;
    }

    Ok(())
}
