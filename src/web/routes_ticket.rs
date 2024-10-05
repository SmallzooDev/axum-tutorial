use axum::extract::State;
use axum::Json;

use crate::model::{ModelContoller, Ticket, TicketForCreate};
use crate::Result;

async fn create_ticket(
    State(mc): State<ModelContoller>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ticket_fc).await?;

    Ok(Json(ticket))
}
