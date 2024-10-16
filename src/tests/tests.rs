// tests/tests.rs

use git_wizard::{create_branch, merge_with_develop, show_git_status};

#[test]
fn test_show_git_status() {
    // Prueba que el estado de git se muestre sin errores
    let result = show_git_status();
    assert!(result.is_ok(), "Expected git status to succeed.");
}

#[test]
fn test_create_branch() {
    // Prueba que se puede crear una rama nueva
    let branch_name = "test-branch";
    let result = create_branch(branch_name);
    assert!(result.is_ok(), "Expected branch creation to succeed.");

    // Limpieza: elimina la rama de prueba después de la prueba
    let _ = std::process::Command::new("git")
        .args(&["branch", "-d", branch_name])
        .output();
}

#[test]
fn test_merge_with_develop() {
    // Primero, asegurémonos de estar en la rama develop
    let _ = std::process::Command::new("git")
        .args(&["checkout", "develop"])
        .output();

    // Crear una rama de prueba para fusionar
    let branch_name = "test-merge-branch";
    let _ = create_branch(branch_name).unwrap();

    // Realiza algunos cambios en la rama de prueba aquí si es necesario

    // Fusiona la rama de prueba con develop
    let result = merge_with_develop();
    assert!(result.is_ok(), "Expected merge with develop to succeed.");

    // Limpieza: elimina la rama de prueba después de la prueba
    let _ = std::process::Command::new("git")
        .args(&["branch", "-d", branch_name])
        .output();
}
