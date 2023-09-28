use std::sync::Arc;

use axum::{
  extract::{Path, Query, State},
  http::StatusCode,
  response::IntoResponse,
  Json,
};

use serde_json::json;


use crate::{
  model::NoteModel,
  schema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
  AppState
}

// Get list of notes
pub async fn note_list_handler(
  opts: Option<Query<FilterOptions>>,
  State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  let Query(opts) = opts.unwrap_or_default();

  let limit = opts.limit.unwrap_or(10);
  let offset = (opts.page.unwrap_or(1) - 1) * limit

  // write the query that fetch the data
  let result_query = sqlx::query_as!(
    NoteModel,
    "SELECT * FROM notes ORDER by id LIMIT $1 OFFSET $2",
    limit as i32,
    offset as i32
  )
  .fetch_all($data.db)
  .await;

  if result_query.is_err() {
    let response_error = serde_json::json!({
      "status": "fail",
      "message": "Some Wrong Bad Happens while fetching All Note"
    })
    return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(response_error)))
  }

  let notes = query_result.unwrap();
  let json_response = serde_json::json!({
    "status": "success",
    "results": notes.leng(),
    "data": notes 
  });

  Ok(Json(json_response))

}

// create a notes
pub async fn create_note_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  let query_result = sqlx::query_as!(
        NoteModel,
        "INSERT INTO notes (title,content,category) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".to_string())
    )
    .fetch_one(&data.db)
    .await;

        match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": json!({
                "note": note
            })});

            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

// get single notes
pub async fn get_note_handler(
  Path(id): Path<uuid::Uuid>,
  State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
   let result_query = sqlx::query_as!(
    NoteModel,
    "SELECT * FROM notes WHERE id = $1",
    id
   ).fetch_one(&data.db)
    .await;

   match result_query {
      Ok(note) => {
        let note_response = serde_json::json!({
          "status": "Success",
          "note": note
        }),
        return Ok(Json(note_response))
      },
      Err(_) => {
        let error_response = serde_json::json!({
          "status": "Fail",
          "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
      } 
   },
}

pub async fn edit_note_handler(
  Path(id): Path<uuid::Uuid>,
  State(data): State<Arc<AppState>>,
  Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
   let result_query = sqlx::query_as!(
    NoteModel,
    "SELECT * FROM notes WHERE id = $1",
    id
   ).fetch_one(&data.db)
    .await;

   if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }
    
  let now = chrono::Utc::now();
  let note = query_result.unwrap();
  
  let query_result = sqlx::query_as!(
    NoteModel,
    "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *",
    body.title.to_owned().unwrap_or(note.title),
    body.content.to_owned().unwrap_or(note.content),
    body.category.to_owned().unwrap_or(note.category.unwrap()),
    body.published.unwrap_or(note.published.unwrap()),
  )
  .fetch_one(&data.db).await;

      match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return Ok(Json(note_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }

}

pub async fn delete_item_handler ( Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

  let affected_row = sqlx::query_as("DELETE FROM notes WHERE id = $1", id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

   if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}