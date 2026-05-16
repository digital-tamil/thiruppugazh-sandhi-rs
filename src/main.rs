use rayon::prelude::*;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Verse {
    original: String,
    split: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Thirupugazh {
    id: u16,
    url: String,
    rhythm: String,
    verses: Vec<Verse>,
}

fn scrape_thirupugazh(id: u16) -> Option<Thirupugazh> {
    let url = format!("https://mayuragiri.com/thiruppugal{}", id);

    thread::sleep(Duration::from_millis(1012));

    // Fetch the page
    let response = reqwest::blocking::get(&url).ok()?.text().ok()?;
    let document = Html::parse_document(&response);

    // Selectors
    let col_selector = Selector::parse(".wp-block-column").unwrap();
    let p_selector = Selector::parse("p").unwrap();

    let columns: Vec<_> = document.select(&col_selector).collect();
    if columns.len() < 2 {
        return None;
    }

    // Extract lines from columns
    let left_lines: Vec<String> = columns[0]
        .select(&p_selector)
        .map(|p| p.text().collect::<Vec<_>>().join("").trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let right_lines: Vec<String> = columns[1]
        .select(&p_selector)
        .map(|p| p.text().collect::<Vec<_>>().join("").trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if left_lines.is_empty() || right_lines.is_empty() {
        return None;
    }

    let rhythm = left_lines[0].clone();
    let mut verses = Vec::new();

    // Skip the rhythm line and zip them
    for (orig, split) in left_lines.iter().skip(1).zip(right_lines.iter().skip(1)) {
        verses.push(Verse {
            original: orig.clone(),
            split: split.clone(),
        });
    }

    println!("Successfully scraped song {}", id);
    Some(Thirupugazh {
        id,
        url,
        rhythm,
        verses,
    })
}

fn main() {
    let start_time = Instant::now();
    const TOTAL_SONGS: u16 = 1338;

    let all_songs = Arc::new(Mutex::new(Vec::new()));

    println!("Starting parallel scrape of {} pages...", TOTAL_SONGS);

    // Use Rayon to parallelize the range 1..1338
    (1..=TOTAL_SONGS).into_par_iter().for_each(|id| {
        if let Some(song) = scrape_thirupugazh(id) {
            let mut data = all_songs.lock().unwrap();
            data.push(song);
        } else {
            eprintln!("Could not scrap song {:02}", id);
        }
    });

    // Sort by ID before saving (since parallel execution is out of order)
    let mut final_data = Arc::try_unwrap(all_songs).unwrap().into_inner().unwrap();
    final_data.sort_by_key(|s| s.id);

    // Save to JSON
    let json = serde_json::to_string_pretty(&final_data).unwrap();
    std::fs::write("data/thiruppugazh.json", json).expect("Unable to write file");

    let duration = start_time.elapsed();
    println!(
        "Finished! Scraped {} songs in {:?}",
        final_data.len(),
        duration
    );
}
