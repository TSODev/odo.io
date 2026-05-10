
async fn list_authorized_vehicles(
    user: ClaimsUser,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<VehicleWithAccess>>, StatusCode> {
    
    let vehicles = sqlx::query_as!(
        VehicleWithAccess,
        r#"
        SELECT v.id, v.make, v.model, v.plate_number, va.role as "my_role: AccessRole"
        FROM public.vehicles v
        JOIN public.vehicle_access va ON v.id = va.vehicle_id
        WHERE va.user_id = $1
        "#,
        user.0.sub
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(vehicles))
}