INSERT INTO public.vehicles (id, make, model, plate_number, owner_id) 
VALUES ('550e8400-e29b-41d4-a716-446655440000', 'Tesla', 'Model 3', 'AA-123-BB', 'user_2p9x...');

INSERT INTO public.vehicle_access (vehicle_id, user_id, role) 
VALUES ('550e8400-e29b-41d4-a716-446655440000', 'user_2p9x...', 'owner');