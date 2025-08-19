use sqlx::{Pool, Sqlite};

use crate::model::Program;
use chrono::NaiveTime;
use dialoguer::Editor;

#[derive(Clone)]
struct EditableProgram {
    id: i64,
    name: String,
    start_time: String,
    end_time: String,
    error: String,
}

pub async fn create_schedule(pool: &Pool<Sqlite>, channel_id: i64) -> Result<(), sqlx::Error> {
    let programs: Vec<Program> =
        sqlx::query_as::<_, Program>("SELECT * FROM programs WHERE channel_id = ? AND enable = 1")
            .bind(channel_id)
            .fetch_all(pool)
            .await?;

    if programs.is_empty() {
        println!("No programs found for this channel.");
        return Ok(());
    }

    let mut editable_rows: Vec<EditableProgram> = programs
        .iter()
        .map(|p| EditableProgram {
            id: p.id,
            name: p.name.clone(),
            start_time: p.start_time.clone(),
            end_time: p.end_time.clone(),
            error: "".to_string(),
        })
        .collect();

    loop {
        // sort by start_time
        editable_rows.sort_by(|a, b| {
            NaiveTime::parse_from_str(&a.start_time, "%H:%M")
                .unwrap_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                .cmp(
                    &NaiveTime::parse_from_str(&b.start_time, "%H:%M")
                        .unwrap_or(NaiveTime::from_hms_opt(0, 0, 0).unwrap()),
                )
        });

        // csv-style editor
        let mut table_text = String::from("name,start_time,end_time,error\n");
        for r in &editable_rows {
            table_text.push_str(&format!(
                "{},{},{},{}\n",
                r.name, r.start_time, r.end_time, r.error
            ));
        }

        println!("Edit schedule. Enter times in HH:MM format (24-hour clock).");

        let edited = Editor::new().edit(&table_text).unwrap();
        if edited.is_none() {
            println!("No changes made.");
            return Ok(());
        }

        let content = edited.unwrap();
        let mut has_error = false;
        let mut new_rows = Vec::new();

        // parse & validate
        let mut times: Vec<(NaiveTime, NaiveTime, usize)> = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if i == 0 {
                continue;
            }
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 4 {
                continue;
            }

            let mut error_msg = String::new();

            let start_time = match NaiveTime::parse_from_str(parts[1], "%H:%M") {
                Ok(t) => t,
                Err(_) => {
                    error_msg.push_str("Invalid start_time; ");
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                }
            };
            let end_time = match NaiveTime::parse_from_str(parts[2], "%H:%M") {
                Ok(t) => t,
                Err(_) => {
                    error_msg.push_str("Invalid end_time; ");
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                }
            };
            if end_time <= start_time {
                error_msg.push_str("end_time before start_time; ");
            }

            times.push((start_time, end_time, i - 1));

            new_rows.push(EditableProgram {
                id: editable_rows[i - 1].id,
                name: parts[0].to_string(),
                start_time: parts[1].to_string(),
                end_time: parts[2].to_string(),
                error: error_msg,
            });
        }

        // overlap check
        for i in 0..times.len() {
            for j in (i + 1)..times.len() {
                let (start_i, end_i, idx_i) = times[i];
                let (start_j, end_j, idx_j) = times[j];
                if start_i < end_j && start_j < end_i {
                    new_rows[idx_i]
                        .error
                        .push_str("Overlaps with another program; ");
                    new_rows[idx_j]
                        .error
                        .push_str("Overlaps with another program; ");
                    has_error = true;
                }
            }
        }

        editable_rows = new_rows;

        if !has_error {
            for r in &editable_rows {
                sqlx::query("UPDATE programs SET start_time = ?, end_time = ? WHERE id = ?")
                    .bind(&r.start_time)
                    .bind(&r.end_time)
                    .bind(r.id)
                    .execute(pool)
                    .await?;
            }
            println!("Schedule updated successfully!");
            break;
        } else {
            println!("Some rows have errors. Please correct them in the editor.\n");
        }
    }

    Ok(())
}
