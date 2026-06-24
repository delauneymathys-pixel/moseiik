use image::io::Reader as ImageReader;
use moseiik::main::{compute_mosaic, Options};

#[cfg(test)]
mod tests {
    use super::*;

    // Test spécifique pour les processeurs d'ordinateurs classiques (PC Windows/Linux)
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86_matches_generic() {
        // 1. On configure les paramètres pour forcer l'algorithme générique (sans accélération)
        let args_generic = Options {
            image: String::from("assets/target-small.png"), // Petite image pour que le test soit instantané
            output: String::from("assets/out-generic-x86.png"),
            tiles: String::from("assets/tiles-small"), // Petit set de vignettes
            scaling: 1,
            tile_size: 5,
            remove_used: false,
            verbose: false,
            simd: false, // <-- TRÈS IMPORTANT : on désactive le SIMD
            num_thread: 4, // On utilise 4 cœurs pour ne pas figer la machine
        };
        compute_mosaic(args_generic); // On génère l'image de référence interne

        // 2. On configure les mêmes paramètres, mais cette fois avec le SIMD activé
        let args_simd = Options {
            image: String::from("assets/target-small.png"),
            output: String::from("assets/out-simd-x86.png"),
            tiles: String::from("assets/tiles-small"),
            scaling: 1,
            tile_size: 5,
            remove_used: false,
            verbose: false,
            simd: true, // <-- TRÈS IMPORTANT : on active le SIMD (SSE2 sur x86)
            num_thread: 4,
        };
        compute_mosaic(args_simd); // On génère l'image accélérée

        // 3. Test différentiel : On recharge les deux images et on vérifie qu'elles sont égales
        // Si le SIMD est bien codé, l'accélération matérielle ne doit pas modifier le résultat mathématique.
        let img_generic = ImageReader::open("assets/out-generic-x86.png").unwrap().decode().unwrap().to_rgb8();
        let img_simd = ImageReader::open("assets/out-simd-x86.png").unwrap().decode().unwrap().to_rgb8();

        assert_eq!(img_generic, img_simd, "Erreur : L'optimisation x86 SSE2 modifie le résultat du calcul !");
    }

    // Test spécifique pour les processeurs ARM (comme les Mac M1/M2/M3 ou les Raspberry Pi)
    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64_matches_generic() {
        // Étape 1 : Génération classique (Sans NEON)
        let args_generic = Options {
            image: String::from("assets/target-small.png"),
            output: String::from("assets/out-generic-arm.png"),
            tiles: String::from("assets/tiles-small"),
            scaling: 1,
            tile_size: 5,
            remove_used: false,
            verbose: false,
            simd: false, // Pas de SIMD
            num_thread: 4,
        };
        compute_mosaic(args_generic);

        // Étape 2 : Génération accélérée (Avec NEON)
        let args_simd = Options {
            image: String::from("assets/target-small.png"),
            output: String::from("assets/out-simd-arm.png"),
            tiles: String::from("assets/tiles-small"),
            scaling: 1,
            tile_size: 5,
            remove_used: false,
            verbose: false,
            simd: true, // Activation de l'accélération matérielle ARM NEON
            num_thread: 4,
        };
        compute_mosaic(args_simd);

        // Étape 3 : Comparaison stricte
        let img_generic = ImageReader::open("assets/out-generic-arm.png").unwrap().decode().unwrap().to_rgb8();
        let img_simd = ImageReader::open("assets/out-simd-arm.png").unwrap().decode().unwrap().to_rgb8();

        assert_eq!(img_generic, img_simd, "Erreur : L'optimisation ARM NEON modifie le résultat du calcul !");
    }

    // Test de sécurité général (S'exécute sur toutes les machines)
    #[test]
    fn test_generic_execution_success() {
        // Ce test s'assure simplement que le programme est capable de lire, calculer et sauvegarder 
        // une image sans planter (Panic) en utilisant l'algorithme de base.
        let args = Options {
            image: String::from("assets/target-small.png"),
            output: String::from("assets/out-generic.png"),
            tiles: String::from("assets/tiles-small"),
            scaling: 1,
            tile_size: 5,
            remove_used: false,
            verbose: false,
            simd: false,
            num_thread: 4,
        };
        
        compute_mosaic(args);

        // On vérifie que le fichier a bien été créé et qu'il n'est pas vide
        let generated = ImageReader::open("assets/out-generic.png").unwrap().decode().unwrap().to_rgb8();
        assert!(generated.width() > 0 && generated.height() > 0, "L'image générée est vide !");
    }
}