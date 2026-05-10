-- 1. On crée un type pour les rôles
CREATE TYPE access_role AS ENUM ('owner', 'editor', 'viewer');

-- 2. Table de gestion des partages
CREATE TABLE public.vehicle_access (
    vehicle_id UUID NOT NULL REFERENCES public.vehicles(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES neon_auth.users(id) ON DELETE CASCADE,
    role access_role NOT NULL DEFAULT 'viewer',
    PRIMARY KEY (vehicle_id, user_id)
);

-- 3. On modifie la table vehicles (optionnel : on peut enlever owner_id 
-- car le 'owner' sera dans vehicle_access)