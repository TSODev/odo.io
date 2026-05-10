-- 1. Activer la RLS sur la table des véhicules
ALTER TABLE public.vehicles ENABLE ROW LEVEL SECURITY;

-- 2. Créer une politique : "L'utilisateur ne peut voir/modifier que ses lignes"
-- On utilise current_setting('request.jwt.claims')::jsonb->>'sub' 
-- qui est une méthode standard pour récupérer l'ID utilisateur dans un jeton JWT avec Neon/Postgres
-- CREATE POLICY vehicle_isolation_policy ON public.vehicles
--     USING (owner_id = (current_setting('request.jwt.claims', true)::jsonb->>'sub'))
--     WITH CHECK (owner_id = (current_setting('request.jwt.claims', true)::jsonb->>'sub'));

-- 3. On fait de même pour les contrats et les kilométrages 
-- en passant par une jointure ou en vérifiant le propriétaire du véhicule lié
ALTER TABLE public.contracts ENABLE ROW LEVEL SECURITY;
CREATE POLICY contract_isolation_policy ON public.contracts
    USING (EXISTS (
        SELECT 1 FROM public.vehicles 
        WHERE vehicles.id = contracts.vehicle_id 
        AND vehicles.owner_id = (current_setting('request.jwt.claims', true)::jsonb->>'sub')
    ));

-- Politique pour les véhicules : Accès si une ligne existe dans vehicle_access
CREATE POLICY vehicle_shared_policy ON public.vehicles
    USING (EXISTS (
        SELECT 1 FROM public.vehicle_access
        WHERE vehicle_access.vehicle_id = id
        AND vehicle_access.user_id = (current_setting('request.jwt.claims', true)::jsonb->>'sub')
    ));    